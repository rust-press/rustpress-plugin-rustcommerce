//! Product Comparison Models
//!
//! Self-contained product comparison functionality.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Comparison list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonList {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub category_id: Option<Uuid>,
    pub products: Vec<ComparisonProduct>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Product in comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonProduct {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub added_at: DateTime<Utc>,
}

/// Comparison table data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonTable {
    pub products: Vec<ComparisonProductData>,
    pub attributes: Vec<ComparisonAttribute>,
    pub highlight_differences: bool,
}

/// Product data for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonProductData {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub image_url: Option<String>,
    pub regular_price: Decimal,
    pub sale_price: Option<Decimal>,
    pub rating: Option<Decimal>,
    pub review_count: i32,
    pub stock_status: StockStatus,
    pub short_description: Option<String>,
    pub category_name: Option<String>,
    pub brand_name: Option<String>,
    pub attribute_values: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StockStatus {
    InStock,
    OutOfStock,
    OnBackorder,
}

/// Comparison attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonAttribute {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub attribute_type: AttributeType,
    pub display_priority: i32,
    pub show_in_comparison: bool,
    pub is_comparable: bool,
    pub highlight_best: bool,
    pub comparison_type: ComparisonType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeType {
    Text,
    Number,
    Boolean,
    Select,
    MultiSelect,
    Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonType {
    HigherBetter,
    LowerBetter,
    Neutral,
}

/// Attribute value for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeValue {
    pub raw_value: String,
    pub display_value: String,
    pub numeric_value: Option<Decimal>,
    pub is_best: bool,
    pub is_worst: bool,
}

/// Comparison widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonWidget {
    pub products: Vec<ComparisonWidgetProduct>,
    pub max_products: i32,
    pub show_add_button: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonWidgetProduct {
    pub product_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub price: Decimal,
}

/// Comparison settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonSettings {
    pub enabled: bool,
    pub max_products: i32,
    pub show_compare_button: bool,
    pub compare_button_position: ButtonPosition,
    pub show_widget: bool,
    pub widget_position: WidgetPosition,
    pub same_category_only: bool,
    pub highlight_differences: bool,
    pub highlight_best_values: bool,
    pub show_similarity_score: bool,
    pub attributes_to_show: Vec<Uuid>,
    pub hidden_attributes: Vec<Uuid>,
    pub persist_comparison: bool,
    pub persistence_days: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonPosition {
    ProductCard,
    ProductPage,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetPosition {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
    Custom,
}

impl Default for ComparisonSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_products: 4,
            show_compare_button: true,
            compare_button_position: ButtonPosition::Both,
            show_widget: true,
            widget_position: WidgetPosition::BottomRight,
            same_category_only: false,
            highlight_differences: true,
            highlight_best_values: true,
            show_similarity_score: false,
            attributes_to_show: Vec::new(),
            hidden_attributes: Vec::new(),
            persist_comparison: true,
            persistence_days: 7,
        }
    }
}

/// Comparison analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    pub total_comparisons: i64,
    pub unique_users: i64,
    pub avg_products_compared: Decimal,
    pub comparisons_to_cart: i64,
    pub comparisons_to_purchase: i64,
    pub conversion_rate: Decimal,

    pub most_compared_products: Vec<ComparedProductStat>,
    pub most_compared_categories: Vec<ComparedCategoryStat>,
    pub common_comparison_pairs: Vec<ComparisonPairStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparedProductStat {
    pub product_id: Uuid,
    pub product_name: String,
    pub comparison_count: i64,
    pub won_count: i64, // How many times chosen after comparison
    pub win_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparedCategoryStat {
    pub category_id: Uuid,
    pub category_name: String,
    pub comparison_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonPairStat {
    pub product_1_id: Uuid,
    pub product_1_name: String,
    pub product_2_id: Uuid,
    pub product_2_name: String,
    pub comparison_count: i64,
}

impl ComparisonList {
    /// Check if a product is in the comparison list
    pub fn contains(&self, product_id: Uuid, variation_id: Option<Uuid>) -> bool {
        self.products.iter().any(|p| {
            p.product_id == product_id && p.variation_id == variation_id
        })
    }

    /// Add product to comparison
    pub fn add_product(&mut self, product_id: Uuid, variation_id: Option<Uuid>, max_products: i32) -> Result<(), ComparisonError> {
        if self.products.len() >= max_products as usize {
            return Err(ComparisonError::MaxProductsReached);
        }

        if self.contains(product_id, variation_id) {
            return Err(ComparisonError::ProductAlreadyAdded);
        }

        self.products.push(ComparisonProduct {
            product_id,
            variation_id,
            added_at: Utc::now(),
        });

        self.updated_at = Some(Utc::now());
        Ok(())
    }

    /// Remove product from comparison
    pub fn remove_product(&mut self, product_id: Uuid, variation_id: Option<Uuid>) -> bool {
        let initial_len = self.products.len();
        self.products.retain(|p| {
            !(p.product_id == product_id && p.variation_id == variation_id)
        });

        if self.products.len() != initial_len {
            self.updated_at = Some(Utc::now());
            true
        } else {
            false
        }
    }

    /// Clear all products
    pub fn clear(&mut self) {
        self.products.clear();
        self.updated_at = Some(Utc::now());
    }

    /// Get product count
    pub fn count(&self) -> usize {
        self.products.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonError {
    MaxProductsReached,
    ProductAlreadyAdded,
    ProductNotFound,
    CategoryMismatch,
}

impl ComparisonTable {
    /// Calculate similarity score between products
    pub fn calculate_similarity(&self, product_1: &ComparisonProductData, product_2: &ComparisonProductData) -> Decimal {
        let mut matching = 0;
        let mut total = 0;

        for attr in &self.attributes {
            if !attr.is_comparable {
                continue;
            }

            let val1 = product_1.attribute_values.get(&attr.slug);
            let val2 = product_2.attribute_values.get(&attr.slug);

            match (val1, val2) {
                (Some(v1), Some(v2)) => {
                    total += 1;
                    if v1.raw_value == v2.raw_value {
                        matching += 1;
                    }
                }
                _ => {}
            }
        }

        if total == 0 {
            return Decimal::ZERO;
        }

        Decimal::from(matching * 100) / Decimal::from(total)
    }
}

/// Add to comparison request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToComparisonRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
}

/// Get comparison request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetComparisonRequest {
    pub product_ids: Vec<Uuid>,
    pub include_attributes: Option<Vec<String>>,
    pub highlight_differences: Option<bool>,
}

/// Comparison response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResponse {
    pub table: ComparisonTable,
    pub similarity_matrix: Option<HashMap<String, Decimal>>,
}
