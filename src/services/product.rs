//! Product Service
//!
//! Handles product management, queries, and product-related operations.

use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::models::product::{
    Product, ProductType, ProductStatus, StockStatus, CatalogVisibility,
    ProductCategory, ProductTag, ProductImage, ProductAttribute, ProductVariation,
    ProductReview, ProductFilter,
};
use crate::services::pricing::PricingService;
use crate::services::inventory::InventoryService;
use crate::settings::RustCommerceSettings;

/// Product service
pub struct ProductService {
    settings: RustCommerceSettings,
}

/// Product error
#[derive(Debug, Clone)]
pub enum ProductError {
    NotFound,
    InvalidData(String),
    DuplicateSku(String),
    InvalidPrice,
    InvalidVariation(String),
}

impl std::fmt::Display for ProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Product not found"),
            Self::InvalidData(msg) => write!(f, "Invalid product data: {}", msg),
            Self::DuplicateSku(sku) => write!(f, "Duplicate SKU: {}", sku),
            Self::InvalidPrice => write!(f, "Invalid price"),
            Self::InvalidVariation(msg) => write!(f, "Invalid variation: {}", msg),
        }
    }
}

/// Product search result
#[derive(Debug, Clone)]
pub struct ProductSearchResult {
    pub products: Vec<Product>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl ProductService {
    /// Create a new product service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Create a new product
    pub fn create_product(
        &self,
        name: String,
        product_type: ProductType,
        status: ProductStatus,
    ) -> Result<Product, ProductError> {
        if name.trim().is_empty() {
            return Err(ProductError::InvalidData("Product name is required".to_string()));
        }

        let slug = self.generate_slug(&name);

        Ok(Product {
            id: Uuid::now_v7(),
            site_id: None,
            name,
            slug,
            product_type,
            status,
            featured: false,
            catalog_visibility: CatalogVisibility::Visible,
            description: None,
            short_description: None,
            sku: None,
            regular_price: None,
            sale_price: None,
            sale_price_from: None,
            sale_price_to: None,
            manage_stock: false,
            stock_quantity: None,
            stock_status: StockStatus::InStock,
            backorders_allowed: false,
            sold_individually: false,
            weight: None,
            length: None,
            width: None,
            height: None,
            shipping_class_id: None,
            reviews_allowed: true,
            average_rating: Decimal::ZERO,
            rating_count: 0,
            parent_id: None,
            menu_order: 0,
            virtual_product: false,
            downloadable: false,
            download_limit: None,
            download_expiry: None,
            tax_status: crate::models::product::TaxStatus::Taxable,
            tax_class: None,
            low_stock_amount: None,
            created_at: Utc::now(),
            updated_at: None,
            published_at: None,
        })
    }

    /// Generate URL slug from name
    pub fn generate_slug(&self, name: &str) -> String {
        name.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Validate product data
    pub fn validate(&self, product: &Product) -> Result<(), ProductError> {
        if product.name.trim().is_empty() {
            return Err(ProductError::InvalidData("Product name is required".to_string()));
        }

        if product.slug.trim().is_empty() {
            return Err(ProductError::InvalidData("Product slug is required".to_string()));
        }

        // Validate prices
        if let Some(regular) = product.regular_price {
            if regular < Decimal::ZERO {
                return Err(ProductError::InvalidPrice);
            }
        }

        if let Some(sale) = product.sale_price {
            if sale < Decimal::ZERO {
                return Err(ProductError::InvalidPrice);
            }

            // Sale price should be less than regular price
            if let Some(regular) = product.regular_price {
                if sale > regular {
                    return Err(ProductError::InvalidData(
                        "Sale price cannot be greater than regular price".to_string()
                    ));
                }
            }
        }

        Ok(())
    }

    /// Check if product is visible
    pub fn is_visible(&self, product: &Product) -> bool {
        if product.status != ProductStatus::Published {
            return false;
        }

        matches!(
            product.catalog_visibility,
            CatalogVisibility::Visible | CatalogVisibility::Catalog
        )
    }

    /// Check if product is searchable
    pub fn is_searchable(&self, product: &Product) -> bool {
        if product.status != ProductStatus::Published {
            return false;
        }

        matches!(
            product.catalog_visibility,
            CatalogVisibility::Visible | CatalogVisibility::Search
        )
    }

    /// Get product variations
    pub fn get_variations(&self, product: &Product, all_variations: &[ProductVariation]) -> Vec<&ProductVariation> {
        if product.product_type != ProductType::Variable {
            return vec![];
        }

        all_variations.iter()
            .filter(|v| v.product_id == product.id)
            .collect()
    }

    /// Create a product variation
    pub fn create_variation(
        &self,
        product: &Product,
        attributes: HashMap<String, String>,
    ) -> Result<ProductVariation, ProductError> {
        if product.product_type != ProductType::Variable {
            return Err(ProductError::InvalidVariation(
                "Cannot create variation for non-variable product".to_string()
            ));
        }

        Ok(ProductVariation {
            id: Uuid::now_v7(),
            product_id: product.id,
            sku: None,
            regular_price: None,
            sale_price: None,
            sale_price_from: None,
            sale_price_to: None,
            manage_stock: false,
            stock_quantity: None,
            stock_status: Some(StockStatus::InStock),
            backorders_allowed: false,
            weight: None,
            length: None,
            width: None,
            height: None,
            attributes,
            image_id: None,
            menu_order: 0,
            is_enabled: true,
            created_at: Utc::now(),
            updated_at: None,
        })
    }

    /// Get variation by attributes
    pub fn find_variation_by_attributes(
        &self,
        variations: &[ProductVariation],
        attributes: &HashMap<String, String>,
    ) -> Option<&ProductVariation> {
        variations.iter().find(|v| {
            attributes.iter().all(|(key, value)| {
                v.attributes.get(key).map_or(false, |v_value| {
                    v_value.is_empty() || v_value == value
                })
            })
        })
    }

    /// Calculate average rating
    pub fn calculate_average_rating(&self, reviews: &[ProductReview]) -> (Decimal, i32) {
        let approved_reviews: Vec<&ProductReview> = reviews.iter()
            .filter(|r| r.is_approved)
            .collect();

        let count = approved_reviews.len() as i32;
        if count == 0 {
            return (Decimal::ZERO, 0);
        }

        let total: i32 = approved_reviews.iter().map(|r| r.rating).sum();
        let average = Decimal::from(total) / Decimal::from(count);

        (average.round_dp(2), count)
    }

    /// Get related products
    pub fn get_related_products(
        &self,
        product: &Product,
        all_products: &[Product],
        category_ids: &[Uuid],
        tag_ids: &[Uuid],
        limit: usize,
    ) -> Vec<&Product> {
        let mut related: Vec<(&Product, i32)> = all_products.iter()
            .filter(|p| {
                p.id != product.id &&
                p.status == ProductStatus::Published &&
                self.is_visible(p)
            })
            .map(|p| {
                let mut score = 0;
                // In full implementation, would check shared categories/tags
                (p, score)
            })
            .collect();

        related.sort_by(|a, b| b.1.cmp(&a.1));
        related.truncate(limit);
        related.into_iter().map(|(p, _)| p).collect()
    }

    /// Get upsell products
    pub fn get_upsells(&self, product: &Product, upsell_ids: &[Uuid], all_products: &[Product]) -> Vec<&Product> {
        upsell_ids.iter()
            .filter_map(|id| {
                all_products.iter()
                    .find(|p| p.id == *id && p.status == ProductStatus::Published)
            })
            .collect()
    }

    /// Get cross-sell products
    pub fn get_cross_sells(&self, product: &Product, cross_sell_ids: &[Uuid], all_products: &[Product]) -> Vec<&Product> {
        cross_sell_ids.iter()
            .filter_map(|id| {
                all_products.iter()
                    .find(|p| p.id == *id && p.status == ProductStatus::Published)
            })
            .collect()
    }

    /// Format product for listing
    pub fn format_product_listing(&self, product: &Product, pricing: &PricingService) -> ProductListing {
        let price_html = if let Some(price) = product.get_price() {
            if product.is_on_sale() {
                if let Some(regular) = product.regular_price {
                    format!(
                        "<del>{}</del> <ins>{}</ins>",
                        pricing.format_price(regular),
                        pricing.format_price(price)
                    )
                } else {
                    pricing.format_price(price)
                }
            } else {
                pricing.format_price(price)
            }
        } else {
            String::new()
        };

        ProductListing {
            id: product.id,
            name: product.name.clone(),
            slug: product.slug.clone(),
            permalink: format!("/product/{}", product.slug),
            price: product.get_price(),
            price_html,
            on_sale: product.is_on_sale(),
            purchasable: product.is_purchasable(),
            featured: product.featured,
            average_rating: product.average_rating,
            review_count: product.rating_count,
        }
    }

    /// Get products on sale
    pub fn get_on_sale_products(&self, products: &[Product]) -> Vec<&Product> {
        products.iter()
            .filter(|p| {
                p.status == ProductStatus::Published &&
                self.is_visible(p) &&
                p.is_on_sale()
            })
            .collect()
    }

    /// Get featured products
    pub fn get_featured_products(&self, products: &[Product]) -> Vec<&Product> {
        products.iter()
            .filter(|p| {
                p.status == ProductStatus::Published &&
                self.is_visible(p) &&
                p.featured
            })
            .collect()
    }

    /// Get best selling products (would use order data)
    pub fn get_best_sellers(&self, products: &[Product], limit: usize) -> Vec<&Product> {
        // In full implementation, would query order items to get sales counts
        products.iter()
            .filter(|p| p.status == ProductStatus::Published && self.is_visible(p))
            .take(limit)
            .collect()
    }

    /// Get new products
    pub fn get_new_products(&self, products: &[Product], limit: usize) -> Vec<&Product> {
        let mut sorted: Vec<&Product> = products.iter()
            .filter(|p| p.status == ProductStatus::Published && self.is_visible(p))
            .collect();

        sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        sorted.truncate(limit);
        sorted
    }

    /// Duplicate a product
    pub fn duplicate(&self, product: &Product) -> Product {
        let mut new_product = product.clone();
        new_product.id = Uuid::now_v7();
        new_product.name = format!("{} (Copy)", product.name);
        new_product.slug = format!("{}-copy", product.slug);
        new_product.status = ProductStatus::Draft;
        new_product.sku = None;
        new_product.created_at = Utc::now();
        new_product.updated_at = None;
        new_product.published_at = None;
        new_product
    }
}

/// Product listing summary
#[derive(Debug, Clone)]
pub struct ProductListing {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub permalink: String,
    pub price: Option<Decimal>,
    pub price_html: String,
    pub on_sale: bool,
    pub purchasable: bool,
    pub featured: bool,
    pub average_rating: Decimal,
    pub review_count: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        let settings = RustCommerceSettings::default();
        let service = ProductService::new(settings);

        assert_eq!(service.generate_slug("Test Product"), "test-product");
        assert_eq!(service.generate_slug("Hello World 123"), "hello-world-123");
        assert_eq!(service.generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_create_product() {
        let settings = RustCommerceSettings::default();
        let service = ProductService::new(settings);

        let product = service.create_product(
            "Test Product".to_string(),
            ProductType::Simple,
            ProductStatus::Draft,
        ).unwrap();

        assert_eq!(product.name, "Test Product");
        assert_eq!(product.slug, "test-product");
        assert_eq!(product.product_type, ProductType::Simple);
    }

    #[test]
    fn test_empty_name_error() {
        let settings = RustCommerceSettings::default();
        let service = ProductService::new(settings);

        let result = service.create_product(
            "".to_string(),
            ProductType::Simple,
            ProductStatus::Draft,
        );

        assert!(matches!(result, Err(ProductError::InvalidData(_))));
    }
}
