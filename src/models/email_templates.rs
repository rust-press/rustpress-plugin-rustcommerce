//! Email Templates System Models
//!
//! Self-contained transactional email templates with variables and localization.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Email template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub template_key: String, // Unique key like "order_confirmation"
    pub name: String,
    pub description: Option<String>,
    pub category: TemplateCategory,
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub content_text: Option<String>,
    pub from_name: Option<String>,
    pub from_email: Option<String>,
    pub reply_to: Option<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub is_enabled: bool,
    pub is_system: bool, // System templates cannot be deleted
    pub variables: Vec<TemplateVariable>,
    pub attachments: Vec<TemplateAttachment>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateCategory {
    // Order related
    OrderConfirmation,
    OrderProcessing,
    OrderShipped,
    OrderDelivered,
    OrderCancelled,
    OrderRefunded,

    // Customer related
    CustomerWelcome,
    CustomerAccountCreated,
    PasswordReset,
    PasswordChanged,
    EmailVerification,

    // Payment
    PaymentReceived,
    PaymentFailed,
    PaymentReminder,
    InvoiceSent,

    // Subscription
    SubscriptionCreated,
    SubscriptionRenewed,
    SubscriptionCancelled,
    SubscriptionExpiring,
    SubscriptionPaymentFailed,

    // Marketing
    AbandonedCart,
    BackInStock,
    PriceDrop,
    WishlistReminder,
    ReviewRequest,
    ReferralInvite,

    // Admin
    LowStockAlert,
    NewOrderNotification,
    RefundRequest,
    CustomerMessage,

    // Other
    Custom,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub variable_type: VariableType,
    pub default_value: Option<String>,
    pub required: bool,
    pub example_value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Number,
    Currency,
    Date,
    DateTime,
    Boolean,
    Url,
    Html,
    List,
    Object,
}

/// Template attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAttachment {
    pub attachment_type: AttachmentType,
    pub filename: Option<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentType {
    Invoice,
    PackingSlip,
    ShippingLabel,
    Custom,
}

/// Email template localization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLocalization {
    pub id: Uuid,
    pub template_id: Uuid,
    pub locale: String, // e.g., "en", "es", "fr"
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub content_text: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Email layout (wrapper template)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailLayout {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub content_html: String,
    pub content_css: Option<String>,
    pub is_default: bool,

    // Branding
    pub logo_url: Option<String>,
    pub logo_width: Option<i32>,
    pub header_color: Option<String>,
    pub footer_color: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub link_color: Option<String>,
    pub button_color: Option<String>,
    pub button_text_color: Option<String>,
    pub font_family: Option<String>,

    // Social links
    pub social_links: Vec<SocialLink>,

    // Footer
    pub footer_text: Option<String>,
    pub show_unsubscribe: bool,
    pub show_address: bool,
    pub company_address: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: SocialPlatform,
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialPlatform {
    Facebook,
    Twitter,
    Instagram,
    LinkedIn,
    YouTube,
    TikTok,
    Pinterest,
    Custom,
}

/// Email snippet (reusable content block)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSnippet {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub snippet_key: String,
    pub content_html: String,
    pub content_text: Option<String>,
    pub variables: Vec<TemplateVariable>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Email send log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSendLog {
    pub id: Uuid,
    pub template_id: Option<Uuid>,
    pub template_key: String,
    pub recipient_email: String,
    pub recipient_name: Option<String>,
    pub subject: String,
    pub status: EmailSendStatus,

    // References
    pub order_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub subscription_id: Option<Uuid>,

    // Tracking
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub bounced_at: Option<DateTime<Utc>>,
    pub complained_at: Option<DateTime<Utc>>,
    pub unsubscribed_at: Option<DateTime<Utc>>,

    pub open_count: i32,
    pub click_count: i32,
    pub clicked_links: Vec<String>,

    pub error_message: Option<String>,
    pub message_id: Option<String>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailSendStatus {
    Queued,
    Sending,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Bounced,
    SoftBounced,
    Complained,
    Unsubscribed,
    Failed,
}

/// Template test data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateTestData {
    pub order: Option<TestOrderData>,
    pub customer: Option<TestCustomerData>,
    pub product: Option<TestProductData>,
    pub subscription: Option<TestSubscriptionData>,
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOrderData {
    pub order_number: String,
    pub order_date: String,
    pub total: String,
    pub subtotal: String,
    pub tax: String,
    pub shipping: String,
    pub discount: String,
    pub items: Vec<TestOrderItem>,
    pub shipping_address: TestAddress,
    pub billing_address: TestAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOrderItem {
    pub name: String,
    pub sku: String,
    pub quantity: i32,
    pub price: String,
    pub total: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAddress {
    pub name: String,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCustomerData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestProductData {
    pub name: String,
    pub sku: String,
    pub price: String,
    pub description: String,
    pub image_url: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSubscriptionData {
    pub subscription_number: String,
    pub plan_name: String,
    pub amount: String,
    pub interval: String,
    pub next_payment_date: String,
}

/// Email template settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplateSettings {
    pub default_from_name: String,
    pub default_from_email: String,
    pub default_reply_to: Option<String>,
    pub default_layout_id: Option<Uuid>,
    pub enable_tracking: bool,
    pub track_opens: bool,
    pub track_clicks: bool,
    pub unsubscribe_url: Option<String>,
    pub company_name: String,
    pub company_address: String,
    pub logo_url: Option<String>,
}

impl Default for EmailTemplateSettings {
    fn default() -> Self {
        Self {
            default_from_name: "My Store".to_string(),
            default_from_email: "noreply@example.com".to_string(),
            default_reply_to: None,
            default_layout_id: None,
            enable_tracking: true,
            track_opens: true,
            track_clicks: true,
            unsubscribe_url: None,
            company_name: "My Company".to_string(),
            company_address: "123 Main St".to_string(),
            logo_url: None,
        }
    }
}

impl EmailTemplate {
    /// Get available variables for this template
    pub fn get_variables(&self) -> &[TemplateVariable] {
        &self.variables
    }

    /// Check if template has required variables
    pub fn validate_variables(&self, provided: &HashMap<String, String>) -> Vec<String> {
        let mut missing = Vec::new();
        for var in &self.variables {
            if var.required && !provided.contains_key(&var.name) {
                missing.push(var.name.clone());
            }
        }
        missing
    }
}

/// Render template request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTemplateRequest {
    pub template_id: Option<Uuid>,
    pub template_key: Option<String>,
    pub locale: Option<String>,
    pub variables: HashMap<String, serde_json::Value>,
    pub layout_id: Option<Uuid>,
}

/// Render template response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTemplateResponse {
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub content_text: Option<String>,
    pub from_name: String,
    pub from_email: String,
}

/// Send email request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    pub template_id: Option<Uuid>,
    pub template_key: Option<String>,
    pub to_email: String,
    pub to_name: Option<String>,
    pub variables: HashMap<String, serde_json::Value>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub attachments: Option<Vec<EmailAttachment>>,
    pub schedule_at: Option<DateTime<Utc>>,

    // Override template settings
    pub subject_override: Option<String>,
    pub from_name_override: Option<String>,
    pub from_email_override: Option<String>,
    pub reply_to_override: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    pub content_base64: String,
}

/// Template analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAnalytics {
    pub template_id: Uuid,
    pub template_key: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    pub total_sent: i64,
    pub total_delivered: i64,
    pub total_opened: i64,
    pub total_clicked: i64,
    pub total_bounced: i64,
    pub total_complained: i64,
    pub total_unsubscribed: i64,

    pub open_rate: f64,
    pub click_rate: f64,
    pub click_to_open_rate: f64,
    pub bounce_rate: f64,
    pub complaint_rate: f64,
    pub unsubscribe_rate: f64,
}

/// Default template definitions
pub fn get_default_templates() -> Vec<EmailTemplate> {
    vec![
        EmailTemplate {
            id: Uuid::nil(),
            site_id: None,
            template_key: "order_confirmation".to_string(),
            name: "Order Confirmation".to_string(),
            description: Some("Sent when an order is placed".to_string()),
            category: TemplateCategory::OrderConfirmation,
            subject: "Order Confirmation - Order #{{order.number}}".to_string(),
            preview_text: Some("Thank you for your order!".to_string()),
            content_html: include_str!("../templates/order_confirmation.html").to_string(),
            content_text: None,
            from_name: None,
            from_email: None,
            reply_to: None,
            cc: vec![],
            bcc: vec![],
            is_enabled: true,
            is_system: true,
            variables: vec![
                TemplateVariable {
                    name: "order".to_string(),
                    description: "Order object".to_string(),
                    variable_type: VariableType::Object,
                    default_value: None,
                    required: true,
                    example_value: None,
                },
                TemplateVariable {
                    name: "customer".to_string(),
                    description: "Customer object".to_string(),
                    variable_type: VariableType::Object,
                    default_value: None,
                    required: true,
                    example_value: None,
                },
            ],
            attachments: vec![],
            version: 1,
            created_at: Utc::now(),
            updated_at: None,
        },
        // Add more default templates as needed
    ]
}
