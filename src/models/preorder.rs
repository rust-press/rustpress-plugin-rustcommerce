//! Pre-order System Models
//!
//! Self-contained pre-order management with deposits and notifications.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use std::collections::HashMap;

/// Pre-order product configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreorderProduct {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub status: PreorderStatus,

    // Dates
    pub availability_date: Option<NaiveDate>,
    pub release_date: Option<NaiveDate>,
    pub preorder_start: Option<DateTime<Utc>>,
    pub preorder_end: Option<DateTime<Utc>>,

    // Inventory
    pub quantity_limit: Option<i32>,
    pub quantity_sold: i32,
    pub remaining_quantity: Option<i32>,

    // Pricing
    pub preorder_price: Option<Decimal>,
    pub deposit_enabled: bool,
    pub deposit_type: DepositType,
    pub deposit_amount: Option<Decimal>,
    pub deposit_percentage: Option<Decimal>,

    // Settings
    pub charge_on_release: bool,
    pub auto_charge_remaining: bool,
    pub allow_cancellation: bool,
    pub cancellation_deadline_days: Option<i32>,

    // Display
    pub show_availability_date: bool,
    pub availability_message: Option<String>,
    pub button_text: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreorderStatus {
    Upcoming,
    Active,
    Ended,
    Released,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepositType {
    Fixed,
    Percentage,
    FullPayment,
}

/// Pre-order record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preorder {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub preorder_product_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub order_id: Uuid,
    pub order_item_id: Uuid,
    pub customer_id: Uuid,
    pub status: PreorderRecordStatus,

    // Pricing
    pub unit_price: Decimal,
    pub quantity: i32,
    pub total_price: Decimal,
    pub deposit_amount: Decimal,
    pub remaining_amount: Decimal,
    pub deposit_paid: bool,
    pub remaining_paid: bool,

    // Dates
    pub expected_date: Option<NaiveDate>,
    pub fulfilled_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,

    // Payment tracking
    pub deposit_payment_id: Option<String>,
    pub remaining_payment_id: Option<String>,

    pub notes: Option<String>,
    pub meta: HashMap<String, String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreorderRecordStatus {
    Pending,
    DepositPaid,
    FullyPaid,
    AwaitingRelease,
    ReadyToShip,
    Shipped,
    Completed,
    Cancelled,
    Refunded,
}

/// Pre-order notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreorderNotification {
    pub id: Uuid,
    pub preorder_id: Uuid,
    pub notification_type: PreorderNotificationType,
    pub status: NotificationStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreorderNotificationType {
    Confirmation,
    ReleaseReminder,
    AvailableNow,
    PaymentReminder,
    Shipped,
    Cancelled,
    DateChanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
}

/// Pre-order settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreorderSettings {
    pub enabled: bool,
    pub default_deposit_type: DepositType,
    pub default_deposit_percentage: Decimal,
    pub allow_mixed_cart: bool, // Allow pre-orders with regular items
    pub auto_convert_out_of_stock: bool,
    pub show_estimated_date: bool,
    pub date_display_format: String,
    pub default_button_text: String,
    pub default_availability_message: String,
    pub send_release_reminder_days: i32,
    pub send_payment_reminder_days: i32,
    pub auto_cancel_unpaid_days: Option<i32>,
}

impl Default for PreorderSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            default_deposit_type: DepositType::Percentage,
            default_deposit_percentage: Decimal::from(25),
            allow_mixed_cart: true,
            auto_convert_out_of_stock: false,
            show_estimated_date: true,
            date_display_format: "MMMM yyyy".to_string(),
            default_button_text: "Pre-order Now".to_string(),
            default_availability_message: "Available {date}".to_string(),
            send_release_reminder_days: 7,
            send_payment_reminder_days: 3,
            auto_cancel_unpaid_days: Some(14),
        }
    }
}

impl PreorderProduct {
    /// Check if pre-order is currently active
    pub fn is_active(&self) -> bool {
        if self.status != PreorderStatus::Active {
            return false;
        }

        let now = Utc::now();

        if let Some(start) = self.preorder_start {
            if now < start {
                return false;
            }
        }

        if let Some(end) = self.preorder_end {
            if now > end {
                return false;
            }
        }

        // Check quantity limit
        if let Some(limit) = self.quantity_limit {
            if self.quantity_sold >= limit {
                return false;
            }
        }

        true
    }

    /// Calculate deposit for a quantity
    pub fn calculate_deposit(&self, unit_price: Decimal, quantity: i32) -> Decimal {
        let total = unit_price * Decimal::from(quantity);

        match self.deposit_type {
            DepositType::FullPayment => total,
            DepositType::Fixed => {
                self.deposit_amount.unwrap_or(Decimal::ZERO) * Decimal::from(quantity)
            }
            DepositType::Percentage => {
                let percentage = self.deposit_percentage.unwrap_or(Decimal::from(25));
                total * percentage / Decimal::from(100)
            }
        }
    }

    /// Get remaining slots
    pub fn remaining_slots(&self) -> Option<i32> {
        self.quantity_limit.map(|limit| (limit - self.quantity_sold).max(0))
    }

    /// Check if release date has passed
    pub fn is_released(&self) -> bool {
        if let Some(release_date) = self.release_date {
            let today = Utc::now().date_naive();
            today >= release_date
        } else {
            false
        }
    }
}

impl Preorder {
    /// Check if can be cancelled
    pub fn can_cancel(&self) -> bool {
        matches!(
            self.status,
            PreorderRecordStatus::Pending
                | PreorderRecordStatus::DepositPaid
                | PreorderRecordStatus::FullyPaid
                | PreorderRecordStatus::AwaitingRelease
        )
    }

    /// Get amount paid
    pub fn amount_paid(&self) -> Decimal {
        if self.remaining_paid {
            self.total_price
        } else if self.deposit_paid {
            self.deposit_amount
        } else {
            Decimal::ZERO
        }
    }

    /// Get refundable amount
    pub fn refundable_amount(&self) -> Decimal {
        self.amount_paid()
    }
}

/// Create pre-order request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePreorderRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub availability_date: Option<NaiveDate>,
    pub release_date: Option<NaiveDate>,
    pub quantity_limit: Option<i32>,
    pub preorder_price: Option<Decimal>,
    pub deposit_type: DepositType,
    pub deposit_amount: Option<Decimal>,
    pub deposit_percentage: Option<Decimal>,
}

/// Pre-order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreorderResponse {
    pub preorder_id: Uuid,
    pub status: PreorderRecordStatus,
    pub deposit_amount: Decimal,
    pub remaining_amount: Decimal,
    pub expected_date: Option<NaiveDate>,
}
