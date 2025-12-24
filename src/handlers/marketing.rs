//! Marketing API Handlers
//!
//! HTTP request handlers for marketing features.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Cart Recovery
#[derive(Debug, Deserialize)]
pub struct AbandonedCartQuery {
    pub status: Option<String>,
    pub recovered: Option<bool>,
    pub min_value: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CartRecoveryStats {
    pub total_abandoned: i64,
    pub total_recovered: i64,
    pub recovery_rate: String,
    pub recovered_revenue: String,
    pub potential_revenue: String,
}

// Email Campaigns
#[derive(Debug, Deserialize)]
pub struct CampaignQuery {
    pub status: Option<String>,
    pub campaign_type: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub campaign_type: String,
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub content_text: Option<String>,
    pub list_ids: Vec<Uuid>,
    pub segment_ids: Vec<Uuid>,
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CampaignStats {
    pub sent: i64,
    pub delivered: i64,
    pub opened: i64,
    pub clicked: i64,
    pub unsubscribed: i64,
    pub bounced: i64,
    pub open_rate: String,
    pub click_rate: String,
}

// Rewards
#[derive(Debug, Deserialize)]
pub struct PointsQuery {
    pub customer_id: Option<Uuid>,
    pub transaction_type: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AdjustPointsRequest {
    pub customer_id: Uuid,
    pub points: i32,
    pub reason: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PointsBalanceResponse {
    pub customer_id: Uuid,
    pub total_points: i64,
    pub redeemable_points: i64,
    pub pending_points: i64,
    pub expiring_soon: i64,
    pub tier: Option<String>,
}

// Recommendations
#[derive(Debug, Deserialize)]
pub struct RecommendationQuery {
    pub product_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub context: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProductRecommendation {
    pub product_id: Uuid,
    pub product_name: String,
    pub product_image: Option<String>,
    pub price: String,
    pub recommendation_type: String,
    pub score: f64,
}

// Dynamic Pricing
#[derive(Debug, Deserialize)]
pub struct PricingRuleQuery {
    pub status: Option<String>,
    pub rule_type: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePricingRuleRequest {
    pub name: String,
    pub rule_type: String,
    pub priority: i32,
    pub conditions: serde_json::Value,
    pub adjustment_type: String,
    pub adjustment_value: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PriceCalculationResponse {
    pub original_price: String,
    pub final_price: String,
    pub discount_amount: String,
    pub applied_rules: Vec<AppliedRule>,
}

#[derive(Debug, Serialize)]
pub struct AppliedRule {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub discount: String,
}
