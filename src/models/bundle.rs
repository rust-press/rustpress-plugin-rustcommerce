//! Product Bundle Models
//!
//! Grouped product bundles with flexible pricing.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Product bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBundle {
    pub id: Uuid,
    pub product_id: Uuid, // The bundle is a product type
    pub bundle_type: BundleType,
    pub layout: BundleLayout,

    // Pricing
    pub pricing_type: BundlePricingType,
    pub bundle_price: Option<Decimal>,
    pub discount_type: Option<BundleDiscountType>,
    pub discount_amount: Option<Decimal>,

    // Settings
    pub min_bundle_size: Option<i32>,
    pub max_bundle_size: Option<i32>,
    pub edit_in_cart: bool,
    pub optional_items: bool,
    pub shipping_type: BundleShippingType,

    // Items
    pub items: Vec<BundleItem>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleType {
    Fixed,      // Fixed set of products
    Mix,        // Customer chooses from allowed products
    Composite,  // Complex configurable bundle
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleLayout {
    Default,
    Grid,
    Tabular,
    Accordion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundlePricingType {
    FixedPrice,    // Bundle has fixed price
    ItemSum,       // Sum of item prices
    ItemSumDiscount, // Sum with discount
    PerItemPricing, // Each item has custom bundle price
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleDiscountType {
    Percentage,
    FixedAmount,
    PerItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleShippingType {
    Bundled,     // Ship as one package
    Individual,  // Each item ships separately
    Calculated,  // Calculate based on contents
}

/// Bundle item (product in bundle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleItem {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,

    // Quantity
    pub quantity_min: i32,
    pub quantity_max: Option<i32>,
    pub quantity_default: i32,

    // Settings
    pub optional: bool,
    pub priced_individually: bool,
    pub shipped_individually: bool,
    pub discount: Option<Decimal>,

    // Visibility
    pub visibility: ItemVisibility,
    pub hide_thumbnail: bool,
    pub override_title: Option<String>,
    pub override_description: Option<String>,

    pub sort_order: i32,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemVisibility {
    Visible,
    Hidden,
    Description, // Show in description only
}

/// Bundle selection (customer's choices for mix bundles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleSelection {
    pub bundle_id: Uuid,
    pub items: Vec<BundleSelectionItem>,
    pub total_price: Decimal,
    pub total_discount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleSelectionItem {
    pub bundle_item_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: i32,
    pub price: Decimal,
    pub discount: Decimal,
}

/// Composite product (advanced bundle with categories)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeProduct {
    pub id: Uuid,
    pub product_id: Uuid,
    pub layout: CompositeLayout,
    pub components: Vec<CompositeComponent>,
    pub scenarios: Vec<CompositeScenario>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompositeLayout {
    Progressive,
    Stepped,
    Componentized,
}

/// Composite component (category of choices)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeComponent {
    pub id: Uuid,
    pub composite_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub query_type: ComponentQueryType,
    pub product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub quantity_min: i32,
    pub quantity_max: i32,
    pub priced_individually: bool,
    pub shipped_individually: bool,
    pub discount: Option<Decimal>,
    pub optional: bool,
    pub default_product_id: Option<Uuid>,
    pub thumbnail_id: Option<Uuid>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentQueryType {
    ProductIds,
    CategoryIds,
}

/// Composite scenario (conditional logic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeScenario {
    pub id: Uuid,
    pub composite_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub conditions: Vec<ScenarioCondition>,
    pub actions: Vec<ScenarioAction>,
    pub is_enabled: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioCondition {
    pub component_id: Uuid,
    pub condition_type: ConditionType,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    ProductSelected,
    ProductNotSelected,
    QuantityEquals,
    QuantityGreater,
    QuantityLess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioAction {
    pub component_id: Uuid,
    pub action_type: ActionType,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Hide,
    Show,
    Disable,
    Enable,
    SetQuantity,
    LimitProducts,
}

impl ProductBundle {
    /// Calculate bundle price
    pub fn calculate_price(&self, selections: &[BundleSelectionItem]) -> Decimal {
        match self.pricing_type {
            BundlePricingType::FixedPrice => {
                self.bundle_price.unwrap_or(Decimal::ZERO)
            }
            BundlePricingType::ItemSum => {
                selections.iter().map(|s| s.price * Decimal::from(s.quantity)).sum()
            }
            BundlePricingType::ItemSumDiscount => {
                let sum: Decimal = selections.iter()
                    .map(|s| s.price * Decimal::from(s.quantity))
                    .sum();
                self.apply_discount(sum)
            }
            BundlePricingType::PerItemPricing => {
                selections.iter().map(|s| s.price * Decimal::from(s.quantity)).sum()
            }
        }
    }

    /// Apply bundle discount
    fn apply_discount(&self, total: Decimal) -> Decimal {
        match (self.discount_type, self.discount_amount) {
            (Some(BundleDiscountType::Percentage), Some(amount)) => {
                total * (Decimal::ONE - amount / Decimal::from(100))
            }
            (Some(BundleDiscountType::FixedAmount), Some(amount)) => {
                (total - amount).max(Decimal::ZERO)
            }
            _ => total,
        }
    }

    /// Validate bundle selection
    pub fn validate_selection(&self, selections: &[BundleSelectionItem]) -> Vec<BundleValidationError> {
        let mut errors = Vec::new();

        // Check min/max bundle size
        let total_qty: i32 = selections.iter().map(|s| s.quantity).sum();

        if let Some(min) = self.min_bundle_size {
            if total_qty < min {
                errors.push(BundleValidationError::BelowMinimum { min, actual: total_qty });
            }
        }

        if let Some(max) = self.max_bundle_size {
            if total_qty > max {
                errors.push(BundleValidationError::AboveMaximum { max, actual: total_qty });
            }
        }

        // Check required items
        for item in &self.items {
            if !item.optional {
                let selected = selections.iter()
                    .find(|s| s.bundle_item_id == item.id);

                if selected.is_none() || selected.map_or(true, |s| s.quantity < item.quantity_min) {
                    errors.push(BundleValidationError::RequiredItemMissing {
                        item_id: item.id,
                        min_qty: item.quantity_min,
                    });
                }
            }
        }

        errors
    }
}

#[derive(Debug, Clone)]
pub enum BundleValidationError {
    BelowMinimum { min: i32, actual: i32 },
    AboveMaximum { max: i32, actual: i32 },
    RequiredItemMissing { item_id: Uuid, min_qty: i32 },
    InvalidProduct { product_id: Uuid },
    OutOfStock { product_id: Uuid },
}
