//! Abandoned Cart Recovery Models
//!
//! Self-contained cart recovery system with email sequences and tracking.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Abandoned cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonedCart {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub cart_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub email: Option<String>,
    pub status: AbandonedCartStatus,

    // Cart details
    pub cart_total: Decimal,
    pub item_count: i32,
    pub items: Vec<AbandonedCartItem>,
    pub currency: String,
    pub coupon_code: Option<String>,

    // Customer info
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,

    // Recovery tracking
    pub checkout_step: CheckoutStep,
    pub abandonment_reason: Option<AbandonmentReason>,
    pub recovery_emails_sent: i32,
    pub last_email_sent_at: Option<DateTime<Utc>>,
    pub recovery_url: Option<String>,
    pub recovery_token: Option<String>,

    // Conversion
    pub recovered: bool,
    pub recovered_at: Option<DateTime<Utc>>,
    pub recovered_order_id: Option<Uuid>,
    pub recovered_value: Option<Decimal>,

    // Timing
    pub cart_created_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub abandoned_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,

    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbandonedCartStatus {
    Abandoned,
    RecoveryStarted,
    EmailSent,
    Recovered,
    Lost,
    Expired,
    Unsubscribed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutStep {
    Cart,
    CustomerInfo,
    ShippingAddress,
    ShippingMethod,
    PaymentMethod,
    Review,
    PaymentProcessing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbandonmentReason {
    Unknown,
    HighShipping,
    PriceComparing,
    JustBrowsing,
    PaymentIssue,
    TechnicalError,
    CouponNotWorking,
    DeliveryTooSlow,
    AccountRequired,
    SecurityConcern,
    ChangedMind,
}

/// Abandoned cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonedCartItem {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub product_name: String,
    pub product_image: Option<String>,
    pub product_url: Option<String>,
    pub quantity: i32,
    pub price: Decimal,
    pub total: Decimal,
    pub attributes: HashMap<String, String>,
}

/// Recovery email sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySequence {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub is_default: bool,
    pub emails: Vec<RecoveryEmail>,
    pub conditions: Vec<SequenceCondition>,

    // Statistics
    pub total_sent: i32,
    pub total_recovered: i32,
    pub total_revenue: Decimal,
    pub conversion_rate: Decimal,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Recovery email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryEmail {
    pub id: Uuid,
    pub sequence_id: Uuid,
    pub email_number: i32, // 1st, 2nd, 3rd email
    pub delay_minutes: i32,
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub content_text: Option<String>,

    // Incentive
    pub include_coupon: bool,
    pub coupon_type: Option<CouponType>,
    pub coupon_value: Option<Decimal>,
    pub coupon_expires_hours: Option<i32>,

    pub is_enabled: bool,

    // Statistics
    pub emails_sent: i32,
    pub emails_opened: i32,
    pub emails_clicked: i32,
    pub conversions: i32,
    pub revenue_generated: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouponType {
    Percentage,
    FixedAmount,
    FreeShipping,
}

/// Sequence condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceCondition {
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    CartValue,
    ItemCount,
    CustomerType, // New vs returning
    CheckoutStep,
    ProductCategory,
    CustomerTag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
}

/// Recovery email send record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryEmailSend {
    pub id: Uuid,
    pub abandoned_cart_id: Uuid,
    pub recovery_email_id: Uuid,
    pub email: String,
    pub subject: String,
    pub status: EmailStatus,

    // Coupon if generated
    pub coupon_code: Option<String>,
    pub coupon_value: Option<Decimal>,
    pub coupon_expires_at: Option<DateTime<Utc>>,

    // Tracking
    pub sent_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub converted_at: Option<DateTime<Utc>>,
    pub order_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailStatus {
    Scheduled,
    Sending,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Converted,
    Bounced,
    Failed,
}

/// Exit intent popup settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitIntentSettings {
    pub enabled: bool,
    pub show_on_cart: bool,
    pub show_on_checkout: bool,
    pub delay_seconds: i32,
    pub show_once_per_session: bool,
    pub show_to_logged_in_only: bool,

    // Content
    pub title: String,
    pub message: String,
    pub email_placeholder: String,
    pub button_text: String,

    // Incentive
    pub offer_discount: bool,
    pub discount_type: Option<CouponType>,
    pub discount_value: Option<Decimal>,

    // Design
    pub background_color: String,
    pub text_color: String,
    pub button_color: String,
    pub image_url: Option<String>,
}

impl Default for ExitIntentSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_on_cart: true,
            show_on_checkout: true,
            delay_seconds: 0,
            show_once_per_session: true,
            show_to_logged_in_only: false,
            title: "Wait! Before you go...".to_string(),
            message: "Complete your purchase and get 10% off!".to_string(),
            email_placeholder: "Enter your email".to_string(),
            button_text: "Get My Discount".to_string(),
            offer_discount: true,
            discount_type: Some(CouponType::Percentage),
            discount_value: Some(Decimal::from(10)),
            background_color: "#ffffff".to_string(),
            text_color: "#333333".to_string(),
            button_color: "#4CAF50".to_string(),
            image_url: None,
        }
    }
}

/// Cart recovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartRecoverySettings {
    pub enabled: bool,
    pub abandonment_threshold_minutes: i32,
    pub cart_expiry_days: i32,
    pub track_guest_carts: bool,
    pub require_email: bool,
    pub capture_email_at_checkout_step: CheckoutStep,

    // Recovery window
    pub max_emails_per_cart: i32,
    pub max_recovery_days: i32,

    // Exclusions
    pub exclude_zero_value_carts: bool,
    pub min_cart_value: Option<Decimal>,
    pub exclude_product_ids: Vec<Uuid>,
    pub exclude_customer_tags: Vec<String>,

    // Unsubscribe handling
    pub respect_marketing_consent: bool,
    pub add_unsubscribe_link: bool,
}

impl Default for CartRecoverySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            abandonment_threshold_minutes: 60,
            cart_expiry_days: 30,
            track_guest_carts: true,
            require_email: false,
            capture_email_at_checkout_step: CheckoutStep::CustomerInfo,
            max_emails_per_cart: 3,
            max_recovery_days: 7,
            exclude_zero_value_carts: true,
            min_cart_value: None,
            exclude_product_ids: Vec::new(),
            exclude_customer_tags: Vec::new(),
            respect_marketing_consent: true,
            add_unsubscribe_link: true,
        }
    }
}

/// Cart recovery analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartRecoveryAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Cart stats
    pub total_abandoned_carts: i64,
    pub total_abandoned_value: Decimal,
    pub avg_abandoned_cart_value: Decimal,

    // Recovery stats
    pub carts_recoverable: i64, // Have email
    pub emails_sent: i64,
    pub emails_opened: i64,
    pub emails_clicked: i64,
    pub carts_recovered: i64,
    pub recovered_value: Decimal,

    // Rates
    pub recovery_rate: Decimal,
    pub open_rate: Decimal,
    pub click_rate: Decimal,
    pub email_to_recovery_rate: Decimal,

    // By email number
    pub stats_by_email: Vec<EmailStats>,

    // By checkout step
    pub abandonment_by_step: HashMap<String, i64>,

    // Top reasons
    pub top_abandonment_reasons: Vec<ReasonStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailStats {
    pub email_number: i32,
    pub sent: i64,
    pub opened: i64,
    pub clicked: i64,
    pub converted: i64,
    pub revenue: Decimal,
    pub conversion_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonStat {
    pub reason: AbandonmentReason,
    pub count: i64,
    pub percentage: Decimal,
}

impl AbandonedCart {
    /// Check if cart is recoverable
    pub fn is_recoverable(&self) -> bool {
        self.email.is_some()
            && matches!(
                self.status,
                AbandonedCartStatus::Abandoned | AbandonedCartStatus::RecoveryStarted | AbandonedCartStatus::EmailSent
            )
            && !self.recovered
    }

    /// Check if more emails can be sent
    pub fn can_send_email(&self, max_emails: i32) -> bool {
        self.is_recoverable() && self.recovery_emails_sent < max_emails
    }

    /// Get hours since abandonment
    pub fn hours_since_abandonment(&self) -> i64 {
        (Utc::now() - self.abandoned_at).num_hours()
    }

    /// Get potential recovery value
    pub fn potential_value(&self) -> Decimal {
        self.cart_total
    }
}

impl RecoverySequence {
    /// Calculate overall conversion rate
    pub fn calculate_conversion_rate(&self) -> Decimal {
        if self.total_sent == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.total_recovered * 100) / Decimal::from(self.total_sent)
    }

    /// Calculate revenue per email
    pub fn revenue_per_email(&self) -> Decimal {
        if self.total_sent == 0 {
            return Decimal::ZERO;
        }
        self.total_revenue / Decimal::from(self.total_sent)
    }
}

/// Capture email request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureEmailRequest {
    pub cart_id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub marketing_consent: Option<bool>,
}

/// Recover cart request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverCartRequest {
    pub token: String,
    pub email: String,
}

/// Recovery cart response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverCartResponse {
    pub success: bool,
    pub cart_id: Option<Uuid>,
    pub cart_url: Option<String>,
    pub coupon_code: Option<String>,
    pub message: String,
}
