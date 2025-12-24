//! Rewards Admin Module
//!
//! Admin handlers for points and rewards program.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Points transaction filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PointsFilters {
    pub customer_id: Option<Uuid>,
    pub transaction_type: Option<String>,
    pub event_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Points balance admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsBalanceView {
    pub customer_id: Uuid,
    pub customer_name: String,
    pub customer_email: String,
    pub balance: i64,
    pub lifetime_earned: i64,
    pub lifetime_redeemed: i64,
}

/// Points transaction admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionView {
    pub id: Uuid,
    pub customer_name: String,
    pub points: i64,
    pub transaction_type: String,
    pub event_type: String,
    pub description: String,
    pub created_at: String,
}

/// Earning rule admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningRuleView {
    pub id: Uuid,
    pub name: String,
    pub event_type: String,
    pub earn_type: String,
    pub points_amount: i64,
    pub is_enabled: bool,
}

/// Referral admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralView {
    pub id: Uuid,
    pub referrer_name: String,
    pub referee_email: String,
    pub status: String,
    pub referrer_points: Option<i64>,
    pub created_at: String,
}
