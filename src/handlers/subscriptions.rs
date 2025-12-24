//! Subscriptions API Handlers
//!
//! HTTP request handlers for subscription management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubscriptionQuery {
    pub status: Option<String>,
    pub customer_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub billing_period: String,
    pub billing_interval: i32,
    pub payment_method: String,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionActionRequest {
    pub action: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub id: Uuid,
    pub status: String,
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub billing_period: String,
    pub next_payment_date: Option<String>,
}
