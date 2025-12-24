//! Coupon Models

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Discount type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    #[default]
    Percent,
    FixedCart,
    FixedProduct,
    PercentProduct,
}

impl DiscountType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Percent => "Percentage discount",
            Self::FixedCart => "Fixed cart discount",
            Self::FixedProduct => "Fixed product discount",
            Self::PercentProduct => "Percentage product discount",
        }
    }

    pub fn is_product_level(&self) -> bool {
        matches!(self, Self::FixedProduct | Self::PercentProduct)
    }
}

/// Coupon entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: Uuid,
    pub site_id: Option<Uuid>,

    // Basic info
    pub code: String,
    pub description: Option<String>,
    pub status: CouponStatus,

    // Discount
    pub discount_type: DiscountType,
    pub amount: Decimal,

    // Usage restrictions
    pub individual_use: bool,
    pub product_ids: Vec<Uuid>,
    pub excluded_product_ids: Vec<Uuid>,
    pub category_ids: Vec<Uuid>,
    pub excluded_category_ids: Vec<Uuid>,

    // Usage limits
    pub usage_limit: Option<i32>,
    pub usage_limit_per_user: Option<i32>,
    pub limit_usage_to_x_items: Option<i32>,
    pub usage_count: i32,

    // Amount restrictions
    pub minimum_amount: Option<Decimal>,
    pub maximum_amount: Option<Decimal>,

    // Free shipping
    pub free_shipping: bool,

    // Dates
    pub date_expires: Option<DateTime<Utc>>,

    // Exclude sale items
    pub exclude_sale_items: bool,

    // Metadata
    pub meta: serde_json::Value,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,

    // Related data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<CouponUsage>>,
}

impl Coupon {
    /// Check if coupon is valid (not expired, not at limit)
    pub fn is_valid(&self) -> CouponValidationResult {
        // Check status
        if self.status != CouponStatus::Publish {
            return CouponValidationResult::Invalid("Coupon is not active".to_string());
        }

        // Check expiration
        if let Some(expires) = self.date_expires {
            if Utc::now() > expires {
                return CouponValidationResult::Invalid("Coupon has expired".to_string());
            }
        }

        // Check usage limit
        if let Some(limit) = self.usage_limit {
            if self.usage_count >= limit {
                return CouponValidationResult::Invalid("Coupon usage limit reached".to_string());
            }
        }

        CouponValidationResult::Valid
    }

    /// Check if coupon can be used by a specific user
    pub fn is_valid_for_user(&self, user_email: &str, user_usage_count: i32) -> CouponValidationResult {
        if let Some(limit) = self.usage_limit_per_user {
            if user_usage_count >= limit {
                return CouponValidationResult::Invalid(
                    "You have already used this coupon the maximum number of times".to_string()
                );
            }
        }

        CouponValidationResult::Valid
    }

    /// Check if coupon applies to a product
    pub fn applies_to_product(&self, product_id: Uuid, category_ids: &[Uuid], is_on_sale: bool) -> bool {
        // Check excluded products
        if self.excluded_product_ids.contains(&product_id) {
            return false;
        }

        // Check excluded categories
        for cat_id in category_ids {
            if self.excluded_category_ids.contains(cat_id) {
                return false;
            }
        }

        // Check sale items
        if self.exclude_sale_items && is_on_sale {
            return false;
        }

        // Check included products (if any specified)
        if !self.product_ids.is_empty() && !self.product_ids.contains(&product_id) {
            return false;
        }

        // Check included categories (if any specified)
        if !self.category_ids.is_empty() {
            let has_category = category_ids.iter().any(|c| self.category_ids.contains(c));
            if !has_category {
                return false;
            }
        }

        true
    }

    /// Calculate discount for a given amount
    pub fn calculate_discount(&self, subtotal: Decimal, item_count: Option<i32>) -> Decimal {
        let discount = match self.discount_type {
            DiscountType::Percent | DiscountType::PercentProduct => {
                let percent = self.amount / Decimal::from(100);
                subtotal * percent
            }
            DiscountType::FixedCart => self.amount,
            DiscountType::FixedProduct => {
                // Fixed per product, multiply by item count
                let count = item_count.unwrap_or(1);
                let max_items = self.limit_usage_to_x_items.unwrap_or(count);
                let items_to_discount = count.min(max_items);
                self.amount * Decimal::from(items_to_discount)
            }
        };

        // Don't exceed subtotal
        discount.min(subtotal)
    }
}

/// Coupon status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CouponStatus {
    #[default]
    Publish,
    Draft,
    Pending,
    Trash,
}

/// Coupon validation result
#[derive(Debug, Clone)]
pub enum CouponValidationResult {
    Valid,
    Invalid(String),
}

impl CouponValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Invalid(msg) => Some(msg),
            Self::Valid => None,
        }
    }
}

/// Coupon usage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponUsage {
    pub id: Uuid,
    pub coupon_id: Uuid,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub used_by_email: Option<String>,
    pub discount_amount: Decimal,
    pub used_at: DateTime<Utc>,
}

// =============================================================================
// DTOs for API
// =============================================================================

/// Request to create a coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub description: Option<String>,
    pub discount_type: Option<DiscountType>,
    pub amount: Decimal,
    pub individual_use: Option<bool>,
    pub product_ids: Option<Vec<Uuid>>,
    pub excluded_product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub excluded_category_ids: Option<Vec<Uuid>>,
    pub usage_limit: Option<i32>,
    pub usage_limit_per_user: Option<i32>,
    pub limit_usage_to_x_items: Option<i32>,
    pub minimum_amount: Option<Decimal>,
    pub maximum_amount: Option<Decimal>,
    pub free_shipping: Option<bool>,
    pub date_expires: Option<DateTime<Utc>>,
    pub exclude_sale_items: Option<bool>,
    pub meta: Option<serde_json::Value>,
}

/// Request to update a coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCouponRequest {
    pub code: Option<String>,
    pub description: Option<String>,
    pub status: Option<CouponStatus>,
    pub discount_type: Option<DiscountType>,
    pub amount: Option<Decimal>,
    pub individual_use: Option<bool>,
    pub product_ids: Option<Vec<Uuid>>,
    pub excluded_product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub excluded_category_ids: Option<Vec<Uuid>>,
    pub usage_limit: Option<i32>,
    pub usage_limit_per_user: Option<i32>,
    pub limit_usage_to_x_items: Option<i32>,
    pub minimum_amount: Option<Decimal>,
    pub maximum_amount: Option<Decimal>,
    pub free_shipping: Option<bool>,
    pub date_expires: Option<DateTime<Utc>>,
    pub exclude_sale_items: Option<bool>,
    pub meta: Option<serde_json::Value>,
}

/// Coupon filter/query parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CouponFilter {
    pub code: Option<String>,
    pub status: Option<CouponStatus>,
    pub discount_type: Option<DiscountType>,
    pub search: Option<String>,
    pub include: Option<Vec<Uuid>>,
    pub exclude: Option<Vec<Uuid>>,
    pub orderby: Option<CouponOrderBy>,
    pub order: Option<super::product::SortOrder>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CouponOrderBy {
    #[default]
    Date,
    Id,
    Code,
    Amount,
    UsageCount,
}
