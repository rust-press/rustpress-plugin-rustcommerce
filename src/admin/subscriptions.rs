//! Subscriptions Admin Module
//!
//! Admin handlers for subscription management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Subscription list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionFilters {
    pub status: Option<String>,
    pub customer_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Subscription admin view data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionView {
    pub id: Uuid,
    pub subscription_number: String,
    pub customer_name: String,
    pub customer_email: String,
    pub status: String,
    pub total: String,
    pub billing_period: String,
    pub next_payment_date: Option<String>,
    pub created_at: String,
}

/// Admin subscription actions
pub enum SubscriptionAction {
    Activate,
    Pause,
    Resume,
    Cancel,
    Renew,
    UpdatePayment,
}
