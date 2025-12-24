//! Email Marketing Models
//!
//! Self-contained email marketing system with campaigns, lists, and automation.
//! Works independently without external email service providers.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Email subscriber
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub customer_id: Option<Uuid>,
    pub status: SubscriberStatus,

    // Subscription info
    pub source: SubscriptionSource,
    pub ip_address: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub unsubscribed_at: Option<DateTime<Utc>>,

    // Engagement metrics
    pub emails_sent: i32,
    pub emails_opened: i32,
    pub emails_clicked: i32,
    pub last_open_at: Option<DateTime<Utc>>,
    pub last_click_at: Option<DateTime<Utc>>,

    // Custom fields
    pub custom_fields: HashMap<String, String>,
    pub tags: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberStatus {
    Pending,      // Awaiting confirmation
    Subscribed,
    Unsubscribed,
    Bounced,
    Complained,
    Cleaned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSource {
    Website,
    Checkout,
    Popup,
    Import,
    Api,
    Manual,
}

/// Email list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailList {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub list_type: ListType,

    // Stats
    pub subscriber_count: i32,
    pub active_count: i32,
    pub unsubscribed_count: i32,

    // Settings
    pub double_optin: bool,
    pub welcome_email_id: Option<Uuid>,
    pub default_from_name: Option<String>,
    pub default_from_email: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListType {
    Marketing,
    Newsletter,
    Transactional,
    Customers,
    Custom,
}

/// Subscriber list membership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMembership {
    pub id: Uuid,
    pub subscriber_id: Uuid,
    pub list_id: Uuid,
    pub status: MembershipStatus,
    pub subscribed_at: DateTime<Utc>,
    pub unsubscribed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Active,
    Unsubscribed,
    Pending,
}

/// Email campaign
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCampaign {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub campaign_type: CampaignType,
    pub status: CampaignStatus,

    // Content
    pub subject: String,
    pub preview_text: Option<String>,
    pub from_name: String,
    pub from_email: String,
    pub reply_to: Option<String>,
    pub content_html: String,
    pub content_text: Option<String>,

    // Targeting
    pub list_ids: Vec<Uuid>,
    pub segment_ids: Vec<Uuid>,
    pub exclude_list_ids: Vec<Uuid>,
    pub recipient_count: i32,

    // Scheduling
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,

    // Analytics
    pub emails_sent: i32,
    pub emails_delivered: i32,
    pub emails_opened: i32,
    pub unique_opens: i32,
    pub emails_clicked: i32,
    pub unique_clicks: i32,
    pub emails_bounced: i32,
    pub emails_complained: i32,
    pub emails_unsubscribed: i32,

    // Revenue tracking
    pub orders_attributed: i32,
    pub revenue_attributed: Decimal,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignType {
    Regular,
    Automated,
    Transactional,
    AbTest,
    Rss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Sent,
    Paused,
    Cancelled,
    Failed,
}

/// Email template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub template_type: TemplateType,
    pub category: Option<String>,
    pub content_html: String,
    pub content_text: Option<String>,
    pub thumbnail: Option<String>,
    pub variables: Vec<TemplateVariable>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateType {
    Campaign,
    Transactional,
    Automation,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub required: bool,
}

/// Email automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAutomation {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub status: AutomationStatus,
    pub trigger: AutomationTrigger,
    pub trigger_settings: HashMap<String, String>,
    pub steps: Vec<AutomationStep>,
    pub list_ids: Option<Vec<Uuid>>,

    // Stats
    pub total_entered: i32,
    pub total_completed: i32,
    pub currently_active: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationStatus {
    Draft,
    Active,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTrigger {
    // Subscriber triggers
    SubscriberAdded,
    SubscriberTagged,
    SubscriberFieldChanged,

    // E-commerce triggers
    CustomerCreated,
    OrderPlaced,
    OrderCompleted,
    OrderRefunded,
    CartAbandoned,
    ProductPurchased,
    FirstPurchase,
    RepeatedPurchase,

    // Engagement triggers
    EmailOpened,
    LinkClicked,
    NotEngaged,

    // Date triggers
    DateBased,
    Anniversary,
    Birthday,

    // Custom
    ApiTriggered,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationStep {
    pub id: Uuid,
    pub automation_id: Uuid,
    pub step_order: i32,
    pub step_type: StepType,
    pub settings: HashMap<String, String>,

    // For email steps
    pub email_subject: Option<String>,
    pub email_content: Option<String>,
    pub template_id: Option<Uuid>,

    // For delay steps
    pub delay_value: Option<i32>,
    pub delay_unit: Option<DelayUnit>,

    // For condition steps
    pub condition_type: Option<ConditionType>,
    pub condition_value: Option<String>,
    pub yes_step_id: Option<Uuid>,
    pub no_step_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    SendEmail,
    Delay,
    Condition,
    UpdateSubscriber,
    AddTag,
    RemoveTag,
    AddToList,
    RemoveFromList,
    Webhook,
    GoalReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DelayUnit {
    Minutes,
    Hours,
    Days,
    Weeks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    HasTag,
    InList,
    FieldEquals,
    HasOpened,
    HasClicked,
    HasPurchased,
    OrderValueGreater,
    Custom,
}

/// Automation enrollment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationEnrollment {
    pub id: Uuid,
    pub automation_id: Uuid,
    pub subscriber_id: Uuid,
    pub current_step_id: Option<Uuid>,
    pub status: EnrollmentStatus,
    pub next_action_at: Option<DateTime<Utc>>,
    pub completed_steps: Vec<Uuid>,
    pub entered_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentStatus {
    Active,
    Waiting,
    Completed,
    Failed,
    Cancelled,
    GoalReached,
}

/// Email segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSegment {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub segment_type: SegmentType,
    pub conditions: Vec<SegmentCondition>,
    pub condition_match: ConditionMatch,
    pub subscriber_count: i32,
    pub last_calculated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentType {
    Static,
    Dynamic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionMatch {
    All,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentCondition {
    pub field: String,
    pub operator: SegmentOperator,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    IsSet,
    IsNotSet,
    DateBefore,
    DateAfter,
    DateWithin,
}

/// Email send record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSend {
    pub id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub automation_id: Option<Uuid>,
    pub subscriber_id: Uuid,
    pub email: String,
    pub subject: String,
    pub status: SendStatus,

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
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendStatus {
    Queued,
    Sending,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Bounced,
    SoftBounced,
    Complained,
    Failed,
}

/// Email marketing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMarketingSettings {
    pub enabled: bool,
    pub default_from_name: String,
    pub default_from_email: String,
    pub default_reply_to: Option<String>,
    pub company_name: String,
    pub company_address: String,
    pub double_optin: bool,
    pub track_opens: bool,
    pub track_clicks: bool,
    pub track_ecommerce: bool,
    pub unsubscribe_page_id: Option<Uuid>,
    pub gdpr_compliant: bool,
    pub send_limit_per_hour: Option<i32>,
    pub send_limit_per_day: Option<i32>,
}

impl Default for EmailMarketingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            default_from_name: "Store".to_string(),
            default_from_email: "hello@example.com".to_string(),
            default_reply_to: None,
            company_name: "My Company".to_string(),
            company_address: "123 Main St".to_string(),
            double_optin: true,
            track_opens: true,
            track_clicks: true,
            track_ecommerce: true,
            unsubscribe_page_id: None,
            gdpr_compliant: true,
            send_limit_per_hour: Some(1000),
            send_limit_per_day: Some(10000),
        }
    }
}

impl Subscriber {
    /// Check if subscriber is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, SubscriberStatus::Subscribed)
    }

    /// Calculate open rate
    pub fn open_rate(&self) -> Decimal {
        if self.emails_sent == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.emails_opened * 100) / Decimal::from(self.emails_sent)
    }

    /// Calculate click rate
    pub fn click_rate(&self) -> Decimal {
        if self.emails_sent == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.emails_clicked * 100) / Decimal::from(self.emails_sent)
    }

    /// Get engagement score (0-100)
    pub fn engagement_score(&self) -> i32 {
        let open_weight = 40;
        let click_weight = 60;

        let open_score = if self.emails_sent > 0 {
            (self.emails_opened * 100 / self.emails_sent) * open_weight / 100
        } else {
            0
        };

        let click_score = if self.emails_opened > 0 {
            (self.emails_clicked * 100 / self.emails_opened) * click_weight / 100
        } else {
            0
        };

        (open_score + click_score).min(100)
    }
}

impl EmailCampaign {
    /// Calculate open rate
    pub fn open_rate(&self) -> Decimal {
        if self.emails_delivered == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.unique_opens * 100) / Decimal::from(self.emails_delivered)
    }

    /// Calculate click rate
    pub fn click_rate(&self) -> Decimal {
        if self.emails_delivered == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.unique_clicks * 100) / Decimal::from(self.emails_delivered)
    }

    /// Calculate click-to-open rate
    pub fn click_to_open_rate(&self) -> Decimal {
        if self.unique_opens == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.unique_clicks * 100) / Decimal::from(self.unique_opens)
    }

    /// Calculate bounce rate
    pub fn bounce_rate(&self) -> Decimal {
        if self.emails_sent == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.emails_bounced * 100) / Decimal::from(self.emails_sent)
    }

    /// Calculate unsubscribe rate
    pub fn unsubscribe_rate(&self) -> Decimal {
        if self.emails_delivered == 0 {
            return Decimal::ZERO;
        }
        Decimal::from(self.emails_unsubscribed * 100) / Decimal::from(self.emails_delivered)
    }
}

/// Subscribe request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub list_ids: Option<Vec<Uuid>>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<HashMap<String, String>>,
    pub source: Option<SubscriptionSource>,
}

/// Campaign send request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCampaignRequest {
    pub campaign_id: Uuid,
    pub schedule_at: Option<DateTime<Utc>>,
    pub send_immediately: bool,
}
