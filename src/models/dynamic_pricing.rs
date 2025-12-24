//! Dynamic Pricing Models
//!
//! Self-contained dynamic pricing system with rules, schedules, and conditions.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime, Weekday};
use std::collections::HashMap;

/// Dynamic pricing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub rule_type: PricingRuleType,
    pub status: RuleStatus,
    pub priority: i32,

    // Targeting
    pub applies_to: AppliesTo,
    pub product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub brand_ids: Option<Vec<Uuid>>,
    pub exclude_product_ids: Vec<Uuid>,
    pub exclude_sale_items: bool,

    // Price modification
    pub adjustment_type: AdjustmentType,
    pub adjustment_value: Decimal,
    pub min_price: Option<Decimal>,
    pub max_discount: Option<Decimal>,

    // Conditions
    pub conditions: Vec<PricingCondition>,
    pub conditions_match: ConditionMatch,

    // Schedule
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub time_restrictions: Option<Vec<TimeRestriction>>,

    // Usage limits
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub per_customer_limit: Option<i32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PricingRuleType {
    Discount,
    Markup,
    FixedPrice,
    BulkPricing,
    TieredPricing,
    BuyXGetY,
    Bundle,
    TimeBased,
    CustomerBased,
    GeoBased,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleStatus {
    Active,
    Inactive,
    Scheduled,
    Expired,
    Draft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppliesTo {
    AllProducts,
    SpecificProducts,
    SpecificCategories,
    SpecificBrands,
    SpecificTags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdjustmentType {
    PercentageDiscount,
    FixedDiscount,
    PercentageMarkup,
    FixedMarkup,
    FixedPrice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionMatch {
    All,
    Any,
}

/// Pricing condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingCondition {
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub value: String,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    // Cart conditions
    CartTotal,
    CartItemCount,
    CartWeight,
    ProductQuantity,

    // Customer conditions
    CustomerGroup,
    CustomerTag,
    IsLoggedIn,
    IsFirstOrder,
    TotalOrders,
    TotalSpent,

    // Product conditions
    ProductTag,
    ProductAttribute,
    ProductPrice,
    StockLevel,

    // Time conditions
    DayOfWeek,
    TimeOfDay,
    DateRange,

    // Geographic
    CustomerCountry,
    ShippingCountry,
    CustomerPostcode,

    // Device/Source
    DeviceType,
    TrafficSource,
    CouponUsed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Contains,
    NotContains,
    In,
    NotIn,
    Between,
}

/// Time restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    pub days: Vec<Weekday>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

/// Bulk/Tiered pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkPricing {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub tiers: Vec<PriceTier>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTier {
    pub min_quantity: i32,
    pub max_quantity: Option<i32>,
    pub price_type: TierPriceType,
    pub price: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TierPriceType {
    FixedPrice,
    PercentageDiscount,
    FixedDiscount,
}

/// Customer group pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGroupPricing {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub customer_group_id: Uuid,
    pub price_type: CustomerPriceType,
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerPriceType {
    FixedPrice,
    PercentageDiscount,
    FixedDiscount,
}

/// Customer group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGroup {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub auto_assign: bool,
    pub assignment_rules: Vec<GroupAssignmentRule>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAssignmentRule {
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub value: String,
}

/// Sale/Flash sale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub sale_type: SaleType,
    pub status: SaleStatus,

    // Products
    pub applies_to: AppliesTo,
    pub product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub exclude_product_ids: Vec<Uuid>,

    // Discount
    pub discount_type: SaleDiscountType,
    pub discount_value: Decimal,
    pub max_discount: Option<Decimal>,

    // Schedule
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,

    // Flash sale specific
    pub is_flash_sale: bool,
    pub show_countdown: bool,
    pub stock_limit: Option<i32>,
    pub per_customer_limit: Option<i32>,

    // Visibility
    pub show_badge: bool,
    pub badge_text: Option<String>,
    pub banner_image: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaleType {
    Standard,
    Flash,
    Clearance,
    Seasonal,
    MemberOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaleStatus {
    Draft,
    Scheduled,
    Active,
    Ended,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaleDiscountType {
    Percentage,
    FixedAmount,
    FixedPrice,
}

/// Buy X Get Y deal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyXGetY {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub status: RuleStatus,

    // Buy condition
    pub buy_product_ids: Option<Vec<Uuid>>,
    pub buy_category_ids: Option<Vec<Uuid>>,
    pub buy_quantity: i32,

    // Get condition
    pub get_product_ids: Option<Vec<Uuid>>,
    pub get_category_ids: Option<Vec<Uuid>>,
    pub get_quantity: i32,
    pub get_discount_type: SaleDiscountType,
    pub get_discount_value: Decimal,

    // Options
    pub max_uses: Option<i32>,
    pub max_uses_per_order: Option<i32>,
    pub can_repeat: bool, // Buy 2 get 1 x2 = Buy 4 get 2

    // Schedule
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Price history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub regular_price: Decimal,
    pub sale_price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub changed_by: Option<Uuid>,
    pub change_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Dynamic pricing calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceCalculation {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub original_price: Decimal,
    pub final_price: Decimal,
    pub discount_amount: Decimal,
    pub discount_percentage: Decimal,
    pub applied_rules: Vec<AppliedRule>,
    pub sale_id: Option<Uuid>,
    pub is_on_sale: bool,
    pub sale_ends_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedRule {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub rule_type: PricingRuleType,
    pub adjustment: Decimal,
}

/// Dynamic pricing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPricingSettings {
    pub enabled: bool,
    pub show_original_price: bool,
    pub show_savings: bool,
    pub show_percentage_saved: bool,
    pub sale_badge_text: String,
    pub allow_stacking: bool,
    pub max_stackable_rules: i32,
    pub apply_to_variations: bool,
    pub round_prices: bool,
    pub rounding_precision: i32,
    pub cache_prices: bool,
    pub cache_ttl_minutes: i32,
}

impl Default for DynamicPricingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_original_price: true,
            show_savings: true,
            show_percentage_saved: true,
            sale_badge_text: "Sale".to_string(),
            allow_stacking: false,
            max_stackable_rules: 1,
            apply_to_variations: true,
            round_prices: true,
            rounding_precision: 2,
            cache_prices: true,
            cache_ttl_minutes: 60,
        }
    }
}

impl PricingRule {
    /// Check if rule is currently active
    pub fn is_active(&self) -> bool {
        if self.status != RuleStatus::Active {
            return false;
        }

        let now = Utc::now();

        if let Some(start) = self.start_date {
            if now < start {
                return false;
            }
        }

        if let Some(end) = self.end_date {
            if now > end {
                return false;
            }
        }

        // Check usage limit
        if let Some(limit) = self.usage_limit {
            if self.usage_count >= limit {
                return false;
            }
        }

        true
    }

    /// Calculate adjusted price
    pub fn apply_adjustment(&self, original_price: Decimal) -> Decimal {
        let adjusted = match self.adjustment_type {
            AdjustmentType::PercentageDiscount => {
                original_price * (Decimal::ONE - self.adjustment_value / Decimal::from(100))
            }
            AdjustmentType::FixedDiscount => {
                (original_price - self.adjustment_value).max(Decimal::ZERO)
            }
            AdjustmentType::PercentageMarkup => {
                original_price * (Decimal::ONE + self.adjustment_value / Decimal::from(100))
            }
            AdjustmentType::FixedMarkup => original_price + self.adjustment_value,
            AdjustmentType::FixedPrice => self.adjustment_value,
        };

        // Apply min price floor
        let adjusted = if let Some(min) = self.min_price {
            adjusted.max(min)
        } else {
            adjusted
        };

        // Apply max discount cap
        if let Some(max_discount) = self.max_discount {
            let discount = original_price - adjusted;
            if discount > max_discount {
                return original_price - max_discount;
            }
        }

        adjusted
    }
}

impl Sale {
    /// Check if sale is currently active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        matches!(self.status, SaleStatus::Active)
            && now >= self.start_date
            && now <= self.end_date
    }

    /// Get time remaining in seconds
    pub fn time_remaining(&self) -> Option<i64> {
        if self.is_active() {
            Some((self.end_date - Utc::now()).num_seconds().max(0))
        } else {
            None
        }
    }
}

/// Calculate price request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatePriceRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: i32,
    pub customer_id: Option<Uuid>,
    pub customer_group_id: Option<Uuid>,
    pub cart_total: Option<Decimal>,
    pub country: Option<String>,
}

/// Create pricing rule request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePricingRuleRequest {
    pub name: String,
    pub description: Option<String>,
    pub rule_type: PricingRuleType,
    pub applies_to: AppliesTo,
    pub product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub adjustment_type: AdjustmentType,
    pub adjustment_value: Decimal,
    pub conditions: Option<Vec<PricingCondition>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}
