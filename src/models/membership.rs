//! Membership Models
//!
//! Membership plans and member access control.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Membership plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipPlan {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: PlanStatus,

    // Access
    pub access_method: AccessMethod,
    pub access_length: Option<i32>,
    pub access_length_period: Option<AccessPeriod>,
    pub access_start_date: Option<DateTime<Utc>>,
    pub access_end_date: Option<DateTime<Utc>>,

    // Products that grant access
    pub product_ids: Vec<Uuid>,

    // Content restrictions
    pub restricted_content: Vec<ContentRestriction>,

    // Product discounts for members
    pub member_discounts: Vec<MemberDiscount>,

    // Settings
    pub trial_enabled: bool,
    pub trial_length: Option<i32>,
    pub trial_period: Option<AccessPeriod>,

    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    Active,
    Draft,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMethod {
    Unlimited,    // No end date
    FixedLength,  // X days/months/years from purchase
    FixedDates,   // Between specific dates
    Subscription, // Linked to subscription
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessPeriod {
    Day,
    Week,
    Month,
    Year,
}

impl AccessPeriod {
    pub fn to_duration(&self, length: i32) -> Duration {
        match self {
            Self::Day => Duration::days(length as i64),
            Self::Week => Duration::weeks(length as i64),
            Self::Month => Duration::days((length * 30) as i64),
            Self::Year => Duration::days((length * 365) as i64),
        }
    }
}

/// User membership (instance of a plan for a user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMembership {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub plan_id: Uuid,
    pub user_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub subscription_id: Option<Uuid>,
    pub status: MembershipStatus,

    // Dates
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub paused_date: Option<DateTime<Utc>>,
    pub cancelled_date: Option<DateTime<Utc>>,

    // Trial
    pub in_trial: bool,
    pub trial_end_date: Option<DateTime<Utc>>,

    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Active,
    FreeTrial,
    DelayedStart,
    Complimentary,
    Pending,
    Paused,
    Expired,
    Cancelled,
}

/// Content restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRestriction {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub content_type: RestrictedContentType,
    pub content_ids: Vec<Uuid>,
    pub access_type: ContentAccessType,
    pub delay_days: Option<i32>, // Drip content
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictedContentType {
    Post,
    Page,
    Product,
    ProductCategory,
    CustomPostType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentAccessType {
    ViewOnly,
    FullAccess,
    Discount,
    FreeAccess,
}

/// Member discount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberDiscount {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub discount_type: MemberDiscountType,
    pub amount: Decimal,
    pub applies_to: DiscountAppliesTo,
    pub product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub exclude_sale_items: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberDiscountType {
    Percentage,
    FixedAmount,
    FreeShipping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscountAppliesTo {
    AllProducts,
    SpecificProducts,
    SpecificCategories,
}

/// Membership note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipNote {
    pub id: Uuid,
    pub membership_id: Uuid,
    pub content: String,
    pub note_type: MembershipNoteType,
    pub added_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipNoteType {
    AdminNote,
    StatusChange,
    System,
}

impl UserMembership {
    /// Check if membership is currently active
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            MembershipStatus::Active | MembershipStatus::FreeTrial | MembershipStatus::Complimentary
        )
    }

    /// Check if membership has expired
    pub fn is_expired(&self) -> bool {
        if let Some(end_date) = self.end_date {
            Utc::now() > end_date
        } else {
            false
        }
    }

    /// Get days remaining
    pub fn days_remaining(&self) -> Option<i64> {
        self.end_date.map(|end| {
            let duration = end - Utc::now();
            duration.num_days().max(0)
        })
    }

    /// Check if in trial period
    pub fn is_in_trial(&self) -> bool {
        if let Some(trial_end) = self.trial_end_date {
            self.in_trial && Utc::now() < trial_end
        } else {
            false
        }
    }

    /// Can be renewed
    pub fn can_renew(&self) -> bool {
        matches!(
            self.status,
            MembershipStatus::Active | MembershipStatus::Expired | MembershipStatus::Cancelled
        )
    }

    /// Can be paused
    pub fn can_pause(&self) -> bool {
        matches!(
            self.status,
            MembershipStatus::Active | MembershipStatus::FreeTrial
        )
    }
}

/// Access check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCheckResult {
    pub has_access: bool,
    pub reason: Option<AccessDeniedReason>,
    pub membership_ids: Vec<Uuid>,
    pub available_after: Option<DateTime<Utc>>, // For drip content
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessDeniedReason {
    NoMembership,
    MembershipExpired,
    MembershipPaused,
    ContentNotIncluded,
    DripContentLocked,
    TrialExpired,
}
