//! Marketing Admin Module
//!
//! Admin handlers for marketing features including coupons, campaigns, cart recovery.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Coupon list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouponFilters {
    pub status: Option<String>,
    pub discount_type: Option<String>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Coupon admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponView {
    pub id: Uuid,
    pub code: String,
    pub discount_type: String,
    pub amount: Decimal,
    pub usage_count: i32,
    pub usage_limit: Option<i32>,
    pub expires_at: Option<String>,
    pub is_active: bool,
}

/// Campaign list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignFilters {
    pub status: Option<String>,
    pub campaign_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Campaign admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignView {
    pub id: Uuid,
    pub name: String,
    pub campaign_type: String,
    pub status: String,
    pub recipient_count: i32,
    pub sent_count: i32,
    pub open_rate: Decimal,
    pub click_rate: Decimal,
    pub scheduled_at: Option<String>,
}

/// Cart recovery filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CartRecoveryFilters {
    pub status: Option<String>,
    pub recovered: Option<bool>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_value: Option<Decimal>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Abandoned cart admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonedCartView {
    pub id: Uuid,
    pub email: Option<String>,
    pub customer_name: Option<String>,
    pub cart_total: Decimal,
    pub item_count: i32,
    pub status: String,
    pub emails_sent: i32,
    pub recovered: bool,
    pub abandoned_at: String,
}

/// Dynamic pricing rule view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRuleView {
    pub id: Uuid,
    pub name: String,
    pub rule_type: String,
    pub status: String,
    pub adjustment_type: String,
    pub adjustment_value: Decimal,
    pub usage_count: i32,
}

/// Recommendation widget view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationWidgetView {
    pub id: Uuid,
    pub name: String,
    pub placement: String,
    pub recommendation_types: Vec<String>,
    pub impressions: i64,
    pub clicks: i64,
    pub conversions: i64,
    pub is_enabled: bool,
}

/// Subscriber view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberView {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: String,
    pub source: String,
    pub lists: Vec<String>,
    pub created_at: String,
}
