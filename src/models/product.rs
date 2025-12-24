//! Product Models

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Product type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    #[default]
    Simple,
    Variable,
    Grouped,
    External,
    Virtual,
    Downloadable,
    Subscription,
    Bundle,
    Booking,
}

/// Product status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    #[default]
    Draft,
    Pending,
    Private,
    Publish,
    Trash,
}

/// Stock status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StockStatus {
    #[default]
    InStock,
    OutOfStock,
    OnBackorder,
}

/// Backorder status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BackorderStatus {
    #[default]
    No,
    Notify,
    Yes,
}

/// Catalog visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CatalogVisibility {
    #[default]
    Visible,
    Catalog,
    Search,
    Hidden,
}

/// Product entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub site_id: Option<Uuid>,

    // Basic info
    pub sku: Option<String>,
    pub name: String,
    pub slug: String,
    pub product_type: ProductType,
    pub status: ProductStatus,

    // Description
    pub short_description: Option<String>,
    pub description: Option<String>,

    // Pricing
    pub regular_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub sale_price_from: Option<DateTime<Utc>>,
    pub sale_price_to: Option<DateTime<Utc>>,

    // Tax
    pub tax_status: TaxStatus,
    pub tax_class: String,

    // Inventory
    pub manage_stock: bool,
    pub stock_quantity: i32,
    pub stock_status: StockStatus,
    pub backorders: BackorderStatus,
    pub low_stock_amount: Option<i32>,
    pub sold_individually: bool,

    // Shipping
    pub weight: Option<Decimal>,
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,
    pub shipping_class_id: Option<Uuid>,

    // Virtual/Downloadable
    pub is_virtual: bool,
    pub is_downloadable: bool,
    pub download_limit: i32,
    pub download_expiry: i32,

    // External product
    pub external_url: Option<String>,
    pub button_text: Option<String>,

    // Reviews
    pub reviews_allowed: bool,
    pub average_rating: Decimal,
    pub rating_count: i32,

    // Visibility
    pub featured: bool,
    pub catalog_visibility: CatalogVisibility,

    // Parent (for variations/grouped)
    pub parent_id: Option<Uuid>,
    pub menu_order: i32,

    // Purchase
    pub purchase_note: Option<String>,
    pub total_sales: i32,

    // SEO
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,

    // Related data (loaded separately)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<ProductCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ProductTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ProductImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ProductAttribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<ProductVariation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<Vec<ProductDownload>>,
}

impl Product {
    /// Get the effective price (sale price if active, otherwise regular)
    pub fn get_price(&self) -> Option<Decimal> {
        if self.is_on_sale() {
            self.sale_price
        } else {
            self.regular_price
        }
    }

    /// Check if product is on sale
    pub fn is_on_sale(&self) -> bool {
        if self.sale_price.is_none() {
            return false;
        }

        let now = Utc::now();

        // Check date range
        if let Some(from) = self.sale_price_from {
            if now < from {
                return false;
            }
        }

        if let Some(to) = self.sale_price_to {
            if now > to {
                return false;
            }
        }

        true
    }

    /// Check if product is purchasable
    pub fn is_purchasable(&self) -> bool {
        self.status == ProductStatus::Publish
            && self.get_price().is_some()
            && (self.stock_status == StockStatus::InStock
                || self.stock_status == StockStatus::OnBackorder)
    }

    /// Check if product is in stock
    pub fn is_in_stock(&self) -> bool {
        if !self.manage_stock {
            return self.stock_status == StockStatus::InStock;
        }

        self.stock_quantity > 0 || self.backorders != BackorderStatus::No
    }
}

/// Tax status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaxStatus {
    #[default]
    Taxable,
    Shipping,
    None,
}

/// Product category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategory {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub image_id: Option<Uuid>,
    pub display_type: CategoryDisplayType,
    pub menu_order: i32,
    pub count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CategoryDisplayType {
    #[default]
    Default,
    Products,
    Subcategories,
    Both,
}

/// Product tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub count: i32,
}

/// Product image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImage {
    pub id: Uuid,
    pub media_id: Uuid,
    pub position: i32,
    pub is_featured: bool,
    pub src: Option<String>,
    pub alt: Option<String>,
    pub title: Option<String>,
}

/// Product attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAttribute {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub attribute_type: AttributeType,
    pub position: i32,
    pub is_visible: bool,
    pub is_variation: bool,
    pub options: Vec<AttributeTerm>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AttributeType {
    #[default]
    Select,
    Color,
    Image,
    Button,
    Text,
}

/// Attribute term (value)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeTerm {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub menu_order: i32,
    pub count: i32,
    pub color: Option<String>,
    pub image_id: Option<Uuid>,
}

/// Product variation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariation {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: Option<String>,
    pub status: ProductStatus,

    // Pricing
    pub regular_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub sale_price_from: Option<DateTime<Utc>>,
    pub sale_price_to: Option<DateTime<Utc>>,

    // Inventory
    pub manage_stock: Option<bool>,
    pub stock_quantity: Option<i32>,
    pub stock_status: Option<StockStatus>,
    pub backorders: Option<BackorderStatus>,

    // Shipping
    pub weight: Option<Decimal>,
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,

    // Virtual/Downloadable
    pub is_virtual: Option<bool>,
    pub is_downloadable: Option<bool>,

    // Display
    pub description: Option<String>,
    pub image_id: Option<Uuid>,
    pub menu_order: i32,

    // Attributes that define this variation
    pub attributes: Vec<VariationAttribute>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Variation attribute (what makes this variation unique)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationAttribute {
    pub attribute_id: Uuid,
    pub attribute_name: String,
    pub term_id: Option<Uuid>,
    pub term_name: Option<String>,
    pub custom_value: Option<String>,
}

/// Product download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDownload {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub file_url: String,
    pub file_hash: Option<String>,
    pub download_count: i32,
    pub position: i32,
}

/// Product review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReview {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Option<Uuid>,
    pub rating: i32,
    pub title: Option<String>,
    pub content: String,
    pub reviewer_name: Option<String>,
    pub reviewer_email: Option<String>,
    pub status: ReviewStatus,
    pub verified_purchase: bool,
    pub helpful_count: i32,
    pub not_helpful_count: i32,
    pub response: Option<String>,
    pub response_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    #[default]
    Pending,
    Approved,
    Spam,
    Trash,
}

/// Shipping class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClass {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

// =============================================================================
// DTOs for API
// =============================================================================

/// Request to create/update a product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRequest {
    pub sku: Option<String>,
    pub name: String,
    pub slug: Option<String>,
    pub product_type: Option<ProductType>,
    pub status: Option<ProductStatus>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub regular_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub sale_price_from: Option<DateTime<Utc>>,
    pub sale_price_to: Option<DateTime<Utc>>,
    pub tax_status: Option<TaxStatus>,
    pub tax_class: Option<String>,
    pub manage_stock: Option<bool>,
    pub stock_quantity: Option<i32>,
    pub stock_status: Option<StockStatus>,
    pub backorders: Option<BackorderStatus>,
    pub low_stock_amount: Option<i32>,
    pub sold_individually: Option<bool>,
    pub weight: Option<Decimal>,
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,
    pub shipping_class_id: Option<Uuid>,
    pub is_virtual: Option<bool>,
    pub is_downloadable: Option<bool>,
    pub download_limit: Option<i32>,
    pub download_expiry: Option<i32>,
    pub external_url: Option<String>,
    pub button_text: Option<String>,
    pub reviews_allowed: Option<bool>,
    pub featured: Option<bool>,
    pub catalog_visibility: Option<CatalogVisibility>,
    pub parent_id: Option<Uuid>,
    pub menu_order: Option<i32>,
    pub purchase_note: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub categories: Option<Vec<Uuid>>,
    pub tags: Option<Vec<Uuid>>,
    pub images: Option<Vec<ProductImageRequest>>,
    pub attributes: Option<Vec<ProductAttributeRequest>>,
    pub variations: Option<Vec<ProductVariationRequest>>,
    pub downloads: Option<Vec<ProductDownloadRequest>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImageRequest {
    pub media_id: Uuid,
    pub position: Option<i32>,
    pub is_featured: Option<bool>,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAttributeRequest {
    pub attribute_id: Uuid,
    pub position: Option<i32>,
    pub is_visible: Option<bool>,
    pub is_variation: Option<bool>,
    pub term_ids: Option<Vec<Uuid>>,
    pub custom_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariationRequest {
    pub id: Option<Uuid>, // For updates
    pub sku: Option<String>,
    pub regular_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub manage_stock: Option<bool>,
    pub stock_quantity: Option<i32>,
    pub stock_status: Option<StockStatus>,
    pub weight: Option<Decimal>,
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,
    pub is_virtual: Option<bool>,
    pub is_downloadable: Option<bool>,
    pub description: Option<String>,
    pub image_id: Option<Uuid>,
    pub attributes: Vec<VariationAttributeRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationAttributeRequest {
    pub attribute_id: Uuid,
    pub term_id: Option<Uuid>,
    pub custom_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDownloadRequest {
    pub name: String,
    pub file_url: String,
}

/// Product filter/query parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductFilter {
    pub status: Option<ProductStatus>,
    pub product_type: Option<ProductType>,
    pub category_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub featured: Option<bool>,
    pub on_sale: Option<bool>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub stock_status: Option<StockStatus>,
    pub sku: Option<String>,
    pub search: Option<String>,
    pub parent_id: Option<Uuid>,
    pub include: Option<Vec<Uuid>>,
    pub exclude: Option<Vec<Uuid>>,
    pub orderby: Option<ProductOrderBy>,
    pub order: Option<SortOrder>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProductOrderBy {
    #[default]
    Date,
    Id,
    Title,
    Slug,
    Price,
    Popularity,
    Rating,
    MenuOrder,
    Rand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}
