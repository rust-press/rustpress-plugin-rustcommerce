//! Waitlist Functionality Models
//!
//! Self-contained waitlist system for out-of-stock products.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Waitlist entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitlistEntry {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: WaitlistStatus,
    pub quantity: i32,

    // Position tracking
    pub position: i32,
    pub priority: WaitlistPriority,

    // Notification tracking
    pub notified: bool,
    pub notified_at: Option<DateTime<Utc>>,
    pub notification_method: NotificationMethod,

    // Conversion tracking
    pub converted: bool,
    pub converted_at: Option<DateTime<Utc>>,
    pub order_id: Option<Uuid>,

    // Expiry
    pub expires_at: Option<DateTime<Utc>>,

    pub source: WaitlistSource,
    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitlistStatus {
    Waiting,
    Notified,
    Purchased,
    Expired,
    Cancelled,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitlistPriority {
    Normal,
    Vip,
    Member,
    FirstTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationMethod {
    Email,
    Sms,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitlistSource {
    ProductPage,
    QuickView,
    Cart,
    Api,
    Import,
}

/// Product waitlist summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductWaitlist {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub total_waiting: i32,
    pub total_quantity_requested: i32,
    pub total_notified: i32,
    pub total_converted: i32,
    pub conversion_rate: Decimal,
    pub avg_wait_days: Option<f64>,
    pub oldest_entry_at: Option<DateTime<Utc>>,
    pub newest_entry_at: Option<DateTime<Utc>>,
}

/// Waitlist notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitlistNotification {
    pub id: Uuid,
    pub waitlist_entry_id: Uuid,
    pub notification_type: WaitlistNotificationType,
    pub channel: NotificationChannel,
    pub status: NotificationStatus,
    pub recipient: String,
    pub subject: Option<String>,
    pub content: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitlistNotificationType {
    JoinConfirmation,
    BackInStock,
    LowStock,
    Reminder,
    Expiring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Failed,
    Bounced,
}

/// Waitlist settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitlistSettings {
    pub enabled: bool,
    pub show_on_out_of_stock: bool,
    pub show_waitlist_count: bool,
    pub show_position: bool,
    pub require_account: bool,
    pub send_confirmation_email: bool,

    // Notification settings
    pub notify_all_at_once: bool,
    pub notify_batch_size: i32,
    pub notify_delay_minutes: i32, // Between batches
    pub reservation_hours: i32,    // Hours to reserve stock after notification

    // Limits
    pub max_per_customer: Option<i32>,
    pub max_per_product: Option<i32>,
    pub entry_expiry_days: Option<i32>,

    // Display
    pub form_title: String,
    pub form_description: String,
    pub success_message: String,
    pub button_text: String,

    // Priority
    pub prioritize_members: bool,
    pub prioritize_by_order_history: bool,
}

impl Default for WaitlistSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_on_out_of_stock: true,
            show_waitlist_count: true,
            show_position: false,
            require_account: false,
            send_confirmation_email: true,
            notify_all_at_once: false,
            notify_batch_size: 10,
            notify_delay_minutes: 30,
            reservation_hours: 24,
            max_per_customer: Some(1),
            max_per_product: None,
            entry_expiry_days: Some(90),
            form_title: "Get notified when available".to_string(),
            form_description: "Enter your email to be notified when this product is back in stock.".to_string(),
            success_message: "You've been added to the waitlist!".to_string(),
            button_text: "Notify Me".to_string(),
            prioritize_members: true,
            prioritize_by_order_history: true,
        }
    }
}

/// Waitlist analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitlistAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Totals
    pub total_entries: i64,
    pub total_waiting: i64,
    pub total_notified: i64,
    pub total_converted: i64,
    pub total_expired: i64,

    // Rates
    pub conversion_rate: Decimal,
    pub notification_open_rate: Decimal,
    pub notification_click_rate: Decimal,

    // Revenue
    pub revenue_from_waitlist: Decimal,
    pub avg_order_value: Decimal,

    // Time metrics
    pub avg_wait_time_days: f64,
    pub avg_conversion_time_hours: f64,

    // Top products
    pub most_waited_products: Vec<WaitedProductStat>,
    pub best_converting_products: Vec<WaitedProductStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitedProductStat {
    pub product_id: Uuid,
    pub product_name: String,
    pub waiting_count: i64,
    pub converted_count: i64,
    pub conversion_rate: Decimal,
}

impl WaitlistEntry {
    /// Check if entry is still active
    pub fn is_active(&self) -> bool {
        matches!(self.status, WaitlistStatus::Waiting | WaitlistStatus::Notified)
    }

    /// Check if entry has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Check if can be notified
    pub fn can_notify(&self) -> bool {
        matches!(self.status, WaitlistStatus::Waiting) && !self.notified
    }

    /// Get days waiting
    pub fn days_waiting(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }
}

/// Join waitlist request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinWaitlistRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub quantity: Option<i32>,
    pub notification_method: Option<NotificationMethod>,
}

/// Join waitlist response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinWaitlistResponse {
    pub success: bool,
    pub entry_id: Option<Uuid>,
    pub position: Option<i32>,
    pub total_waiting: i32,
    pub message: String,
    pub already_on_waitlist: bool,
}

/// Notify waitlist request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyWaitlistRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub available_quantity: i32,
    pub notify_all: bool,
}

/// Notify waitlist response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyWaitlistResponse {
    pub notified_count: i32,
    pub remaining_waiting: i32,
    pub notifications_sent: Vec<Uuid>,
}
