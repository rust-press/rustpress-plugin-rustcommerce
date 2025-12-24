//! Subscription Models
//!
//! Recurring payment subscriptions for products and services.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub parent_order_id: Uuid,
    pub status: SubscriptionStatus,

    // Billing
    pub billing_period: BillingPeriod,
    pub billing_interval: i32,
    pub trial_period: Option<BillingPeriod>,
    pub trial_length: Option<i32>,

    // Amounts
    pub total: Decimal,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub shipping_total: Decimal,
    pub discount_total: Decimal,
    pub currency: String,

    // Payment
    pub payment_method: String,
    pub payment_token_id: Option<Uuid>,
    pub requires_manual_renewal: bool,

    // Dates
    pub start_date: DateTime<Utc>,
    pub trial_end_date: Option<DateTime<Utc>>,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub cancelled_date: Option<DateTime<Utc>>,

    // Items
    pub items: Vec<SubscriptionItem>,

    // Addresses
    pub billing_address: serde_json::Value,
    pub shipping_address: serde_json::Value,

    // Metadata
    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Subscription status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Pending,
    Active,
    OnHold,
    Cancelled,
    Expired,
    PendingCancel,
    Switched,
}

/// Billing period
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingPeriod {
    Day,
    Week,
    Month,
    Year,
}

impl BillingPeriod {
    pub fn to_duration(&self, interval: i32) -> Duration {
        match self {
            Self::Day => Duration::days(interval as i64),
            Self::Week => Duration::weeks(interval as i64),
            Self::Month => Duration::days((interval * 30) as i64),
            Self::Year => Duration::days((interval * 365) as i64),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}

/// Subscription item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionItem {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub subtotal: Decimal,
    pub total: Decimal,
    pub tax: Decimal,
    pub meta: HashMap<String, String>,
}

/// Subscription product settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionProductSettings {
    pub price: Decimal,
    pub period: BillingPeriod,
    pub interval: i32,
    pub length: Option<i32>, // 0 or None = unlimited
    pub sign_up_fee: Option<Decimal>,
    pub trial_period: Option<BillingPeriod>,
    pub trial_length: Option<i32>,
    pub one_time_shipping: bool,
    pub limit: SubscriptionLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionLimit {
    NoLimit,
    OneActive,
    OneEver,
}

/// Renewal order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewalOrder {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub order_id: Uuid,
    pub renewal_date: DateTime<Utc>,
    pub status: RenewalStatus,
    pub amount: Decimal,
    pub payment_attempts: i32,
    pub last_attempt: Option<DateTime<Utc>>,
    pub next_retry: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenewalStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Subscription schedule change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleChange {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub change_type: ScheduleChangeType,
    pub effective_date: DateTime<Utc>,
    pub old_value: serde_json::Value,
    pub new_value: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleChangeType {
    PlanChange,
    QuantityChange,
    PriceChange,
    DateChange,
    Pause,
    Resume,
}

impl Subscription {
    /// Check if subscription is active
    pub fn is_active(&self) -> bool {
        self.status == SubscriptionStatus::Active
    }

    /// Check if in trial period
    pub fn is_in_trial(&self) -> bool {
        if let Some(trial_end) = self.trial_end_date {
            Utc::now() < trial_end && self.status == SubscriptionStatus::Active
        } else {
            false
        }
    }

    /// Get days until next payment
    pub fn days_until_renewal(&self) -> Option<i64> {
        self.next_payment_date.map(|date| {
            let duration = date - Utc::now();
            duration.num_days()
        })
    }

    /// Calculate next payment date
    pub fn calculate_next_payment(&self) -> DateTime<Utc> {
        let base_date = self.next_payment_date.unwrap_or(Utc::now());
        base_date + self.billing_period.to_duration(self.billing_interval)
    }

    /// Check if can be cancelled
    pub fn can_cancel(&self) -> bool {
        matches!(
            self.status,
            SubscriptionStatus::Active | SubscriptionStatus::OnHold | SubscriptionStatus::Pending
        )
    }

    /// Check if can be reactivated
    pub fn can_reactivate(&self) -> bool {
        matches!(
            self.status,
            SubscriptionStatus::Cancelled | SubscriptionStatus::OnHold | SubscriptionStatus::Expired
        )
    }
}

/// Subscription switch/upgrade request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchRequest {
    pub subscription_id: Uuid,
    pub new_product_id: Uuid,
    pub new_variation_id: Option<Uuid>,
    pub prorate: bool,
    pub effective_immediately: bool,
}

/// Switch calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchCalculation {
    pub credit_amount: Decimal,
    pub new_recurring_amount: Decimal,
    pub amount_due_now: Decimal,
    pub next_payment_date: DateTime<Utc>,
}
