//! Inventory Service
//!
//! Handles stock management, inventory tracking, and low stock notifications.

use rust_decimal::Decimal;
use uuid::Uuid;
use crate::models::product::{Product, ProductVariation, StockStatus};

/// Inventory service
pub struct InventoryService {
    low_stock_threshold: i32,
    manage_stock: bool,
}

/// Stock change type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StockChangeType {
    Sale,
    Refund,
    Restock,
    Adjustment,
    Return,
}

/// Stock change record
#[derive(Debug, Clone)]
pub struct StockChange {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub change_type: StockChangeType,
    pub quantity_change: i32,
    pub previous_quantity: Option<i32>,
    pub new_quantity: Option<i32>,
    pub order_id: Option<Uuid>,
    pub note: Option<String>,
}

/// Inventory check result
#[derive(Debug, Clone)]
pub struct InventoryCheckResult {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub requested_quantity: i32,
    pub available_quantity: Option<i32>,
    pub is_available: bool,
    pub is_backorder: bool,
    pub message: Option<String>,
}

impl InventoryService {
    /// Create a new inventory service
    pub fn new(low_stock_threshold: i32, manage_stock: bool) -> Self {
        Self {
            low_stock_threshold,
            manage_stock,
        }
    }

    /// Check if a product has enough stock
    pub fn check_stock(&self, product: &Product, quantity: i32) -> InventoryCheckResult {
        let product_id = product.id;

        // If stock management is disabled globally
        if !self.manage_stock {
            return InventoryCheckResult {
                product_id,
                variation_id: None,
                requested_quantity: quantity,
                available_quantity: None,
                is_available: true,
                is_backorder: false,
                message: None,
            };
        }

        // If product doesn't manage stock
        if !product.manage_stock {
            return InventoryCheckResult {
                product_id,
                variation_id: None,
                requested_quantity: quantity,
                available_quantity: None,
                is_available: product.stock_status != StockStatus::OutOfStock,
                is_backorder: false,
                message: if product.stock_status == StockStatus::OutOfStock {
                    Some("Out of stock".to_string())
                } else {
                    None
                },
            };
        }

        let stock_qty = product.stock_quantity.unwrap_or(0);

        // Check if enough stock
        if stock_qty >= quantity {
            InventoryCheckResult {
                product_id,
                variation_id: None,
                requested_quantity: quantity,
                available_quantity: Some(stock_qty),
                is_available: true,
                is_backorder: false,
                message: None,
            }
        } else if product.backorders_allowed {
            // Allow backorders
            InventoryCheckResult {
                product_id,
                variation_id: None,
                requested_quantity: quantity,
                available_quantity: Some(stock_qty),
                is_available: true,
                is_backorder: true,
                message: Some(format!("{} on backorder", quantity - stock_qty)),
            }
        } else {
            // Not enough stock
            InventoryCheckResult {
                product_id,
                variation_id: None,
                requested_quantity: quantity,
                available_quantity: Some(stock_qty),
                is_available: false,
                is_backorder: false,
                message: Some(format!("Only {} in stock", stock_qty)),
            }
        }
    }

    /// Check variation stock
    pub fn check_variation_stock(
        &self,
        product: &Product,
        variation: &ProductVariation,
        quantity: i32,
    ) -> InventoryCheckResult {
        let product_id = product.id;
        let variation_id = Some(variation.id);

        // If stock management is disabled globally
        if !self.manage_stock {
            return InventoryCheckResult {
                product_id,
                variation_id,
                requested_quantity: quantity,
                available_quantity: None,
                is_available: true,
                is_backorder: false,
                message: None,
            };
        }

        // Check if variation manages its own stock
        let (manages_stock, stock_qty, backorders_allowed) = if variation.manage_stock {
            (true, variation.stock_quantity, variation.backorders_allowed)
        } else {
            // Fall back to product stock
            (product.manage_stock, product.stock_quantity, product.backorders_allowed)
        };

        if !manages_stock {
            let stock_status = variation.stock_status.unwrap_or(product.stock_status);
            return InventoryCheckResult {
                product_id,
                variation_id,
                requested_quantity: quantity,
                available_quantity: None,
                is_available: stock_status != StockStatus::OutOfStock,
                is_backorder: false,
                message: if stock_status == StockStatus::OutOfStock {
                    Some("Out of stock".to_string())
                } else {
                    None
                },
            };
        }

        let stock = stock_qty.unwrap_or(0);

        if stock >= quantity {
            InventoryCheckResult {
                product_id,
                variation_id,
                requested_quantity: quantity,
                available_quantity: Some(stock),
                is_available: true,
                is_backorder: false,
                message: None,
            }
        } else if backorders_allowed {
            InventoryCheckResult {
                product_id,
                variation_id,
                requested_quantity: quantity,
                available_quantity: Some(stock),
                is_available: true,
                is_backorder: true,
                message: Some(format!("{} on backorder", quantity - stock)),
            }
        } else {
            InventoryCheckResult {
                product_id,
                variation_id,
                requested_quantity: quantity,
                available_quantity: Some(stock),
                is_available: false,
                is_backorder: false,
                message: Some(format!("Only {} in stock", stock)),
            }
        }
    }

    /// Calculate new stock quantity after a change
    pub fn calculate_new_stock(
        &self,
        current_stock: Option<i32>,
        change_type: StockChangeType,
        quantity: i32,
    ) -> Option<i32> {
        let current = current_stock?;

        Some(match change_type {
            StockChangeType::Sale => current - quantity,
            StockChangeType::Refund | StockChangeType::Return | StockChangeType::Restock => {
                current + quantity
            }
            StockChangeType::Adjustment => quantity, // Absolute set
        })
    }

    /// Check if stock is low
    pub fn is_low_stock(&self, product: &Product) -> bool {
        if !product.manage_stock {
            return false;
        }

        let threshold = product.low_stock_amount.unwrap_or(self.low_stock_threshold);
        let stock = product.stock_quantity.unwrap_or(0);

        stock <= threshold && stock > 0
    }

    /// Check if product is out of stock
    pub fn is_out_of_stock(&self, product: &Product) -> bool {
        if !product.manage_stock {
            return product.stock_status == StockStatus::OutOfStock;
        }

        product.stock_quantity.map_or(false, |qty| qty <= 0)
    }

    /// Get stock status based on quantity
    pub fn get_stock_status(&self, product: &Product) -> StockStatus {
        if !product.manage_stock {
            return product.stock_status;
        }

        match product.stock_quantity {
            Some(qty) if qty <= 0 => {
                if product.backorders_allowed {
                    StockStatus::OnBackorder
                } else {
                    StockStatus::OutOfStock
                }
            }
            Some(_) => StockStatus::InStock,
            None => product.stock_status,
        }
    }

    /// Get human-readable stock text
    pub fn get_stock_text(&self, product: &Product) -> String {
        if !product.manage_stock {
            return match product.stock_status {
                StockStatus::InStock => "In stock".to_string(),
                StockStatus::OutOfStock => "Out of stock".to_string(),
                StockStatus::OnBackorder => "Available on backorder".to_string(),
            };
        }

        match product.stock_quantity {
            Some(qty) if qty <= 0 => {
                if product.backorders_allowed {
                    "Available on backorder".to_string()
                } else {
                    "Out of stock".to_string()
                }
            }
            Some(qty) => {
                let threshold = product.low_stock_amount.unwrap_or(self.low_stock_threshold);
                if qty <= threshold {
                    format!("Only {} left in stock", qty)
                } else {
                    format!("{} in stock", qty)
                }
            }
            None => "In stock".to_string(),
        }
    }

    /// Reserve stock for an order (doesn't actually reduce, just validates)
    pub fn reserve_stock(
        &self,
        items: &[(Product, Option<ProductVariation>, i32)],
    ) -> Result<Vec<InventoryCheckResult>, Vec<InventoryCheckResult>> {
        let mut results = Vec::new();
        let mut has_error = false;

        for (product, variation, quantity) in items {
            let result = if let Some(var) = variation {
                self.check_variation_stock(product, var, *quantity)
            } else {
                self.check_stock(product, *quantity)
            };

            if !result.is_available {
                has_error = true;
            }
            results.push(result);
        }

        if has_error {
            Err(results)
        } else {
            Ok(results)
        }
    }

    /// Create a stock change record
    pub fn create_stock_change(
        &self,
        product_id: Uuid,
        variation_id: Option<Uuid>,
        change_type: StockChangeType,
        quantity_change: i32,
        previous_quantity: Option<i32>,
        order_id: Option<Uuid>,
        note: Option<String>,
    ) -> StockChange {
        let new_quantity = self.calculate_new_stock(previous_quantity, change_type, quantity_change);

        StockChange {
            product_id,
            variation_id,
            change_type,
            quantity_change,
            previous_quantity,
            new_quantity,
            order_id,
            note,
        }
    }
}

/// Bulk stock update request
#[derive(Debug, Clone)]
pub struct BulkStockUpdate {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub new_quantity: Option<i32>,
    pub stock_status: Option<StockStatus>,
}

/// Low stock item
#[derive(Debug, Clone)]
pub struct LowStockItem {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub product_name: String,
    pub sku: Option<String>,
    pub current_stock: i32,
    pub threshold: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::product::ProductType;

    fn create_test_product(stock: Option<i32>, manage_stock: bool) -> Product {
        Product {
            id: Uuid::new_v4(),
            site_id: None,
            name: "Test Product".to_string(),
            slug: "test-product".to_string(),
            product_type: ProductType::Simple,
            status: crate::models::product::ProductStatus::Published,
            featured: false,
            catalog_visibility: crate::models::product::CatalogVisibility::Visible,
            description: None,
            short_description: None,
            sku: None,
            regular_price: None,
            sale_price: None,
            sale_price_from: None,
            sale_price_to: None,
            manage_stock,
            stock_quantity: stock,
            stock_status: if stock.map_or(true, |s| s > 0) {
                StockStatus::InStock
            } else {
                StockStatus::OutOfStock
            },
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
            created_at: chrono::Utc::now(),
            updated_at: None,
            published_at: None,
        }
    }

    #[test]
    fn test_check_stock_sufficient() {
        let service = InventoryService::new(5, true);
        let product = create_test_product(Some(10), true);

        let result = service.check_stock(&product, 5);
        assert!(result.is_available);
        assert!(!result.is_backorder);
    }

    #[test]
    fn test_check_stock_insufficient() {
        let service = InventoryService::new(5, true);
        let product = create_test_product(Some(3), true);

        let result = service.check_stock(&product, 5);
        assert!(!result.is_available);
    }

    #[test]
    fn test_low_stock_detection() {
        let service = InventoryService::new(5, true);
        let product = create_test_product(Some(3), true);

        assert!(service.is_low_stock(&product));
    }
}
