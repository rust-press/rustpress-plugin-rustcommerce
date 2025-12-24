//! Points and Rewards Models
//!
//! Loyalty program with points earning and redemption.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Customer points balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsBalance {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub balance: i64,
    pub lifetime_earned: i64,
    pub lifetime_redeemed: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Points transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsTransaction {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub points: i64, // Positive for earn, negative for redeem
    pub transaction_type: PointsTransactionType,
    pub event_type: PointsEventType,

    // Reference
    pub order_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub user_id: Option<Uuid>, // Admin who adjusted

    pub description: String,
    pub data: HashMap<String, String>,

    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PointsTransactionType {
    Earn,
    Redeem,
    Adjust,
    Expire,
    AdminAdjust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PointsEventType {
    // Earning events
    Purchase,
    Signup,
    Review,
    Referral,
    Birthday,
    Custom,
    AdminCredit,

    // Redemption events
    CartDiscount,
    ProductPurchase,
    FreeShipping,
    AdminDebit,

    // Other events
    Expiration,
    OrderCancelled,
    OrderRefunded,
}

/// Points earning rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningRule {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub event_type: PointsEventType,
    pub is_enabled: bool,
    pub priority: i32,

    // Earning amount
    pub earn_type: EarnType,
    pub points_amount: i64,
    pub points_per_currency: Option<Decimal>, // e.g., 1 point per $1

    // Conditions
    pub min_order_total: Option<Decimal>,
    pub max_points_per_order: Option<i64>,
    pub product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub customer_roles: Option<Vec<String>>,

    // Validity
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EarnType {
    Fixed,          // Fixed number of points
    PerCurrency,    // Points per currency spent
    PerProduct,     // Points per product
    Percentage,     // Percentage of order value
}

/// Redemption rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedemptionRule {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub priority: i32,

    // Redemption type
    pub redemption_type: RedemptionType,
    pub points_required: i64,
    pub discount_amount: Decimal, // Currency amount or percentage

    // Limits
    pub min_points_to_redeem: Option<i64>,
    pub max_points_per_order: Option<i64>,
    pub max_discount_percentage: Option<Decimal>,

    // Conditions
    pub min_order_total: Option<Decimal>,
    pub product_ids: Option<Vec<Uuid>>,
    pub category_ids: Option<Vec<Uuid>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedemptionType {
    CartDiscount,
    FixedDiscount,
    PercentageDiscount,
    FreeProduct,
    FreeShipping,
}

/// Points program settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsSettings {
    pub enabled: bool,
    pub singular_name: String, // "point"
    pub plural_name: String,   // "points"

    // Earning
    pub earn_points_ratio: Decimal, // Points per $1
    pub round_points: RoundingMethod,

    // Redemption
    pub points_value: Decimal, // $0.01 per point
    pub min_points_to_redeem: i64,
    pub max_discount_percentage: Option<Decimal>,

    // Expiration
    pub points_expire: bool,
    pub expiration_days: Option<i32>,

    // Display
    pub show_points_on_product: bool,
    pub show_points_on_cart: bool,
    pub show_points_on_checkout: bool,

    // Messages
    pub earn_message_product: String,
    pub earn_message_cart: String,
    pub redeem_message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundingMethod {
    RoundUp,
    RoundDown,
    RoundNearest,
    NoRounding,
}

impl Default for PointsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            singular_name: "point".to_string(),
            plural_name: "points".to_string(),
            earn_points_ratio: Decimal::ONE,
            round_points: RoundingMethod::RoundDown,
            points_value: Decimal::new(1, 2), // 0.01
            min_points_to_redeem: 100,
            max_discount_percentage: None,
            points_expire: false,
            expiration_days: None,
            show_points_on_product: true,
            show_points_on_cart: true,
            show_points_on_checkout: true,
            earn_message_product: "Earn {points} {label} by purchasing this product".to_string(),
            earn_message_cart: "Complete your order to earn {points} {label}".to_string(),
            redeem_message: "Use your {points} {label} for a discount on this order".to_string(),
        }
    }
}

/// Referral program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Referral {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub referrer_id: Uuid,      // Customer who referred
    pub referee_id: Option<Uuid>, // New customer
    pub referee_email: String,
    pub referral_code: String,
    pub status: ReferralStatus,
    pub referrer_points: Option<i64>,
    pub referee_points: Option<i64>,
    pub order_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferralStatus {
    Pending,
    Signed,   // Referee signed up
    Complete, // Referee made purchase
    Credited, // Points awarded
    Expired,
}

/// Points calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsCalculation {
    pub points_to_earn: i64,
    pub points_available: i64,
    pub points_to_redeem: i64,
    pub discount_amount: Decimal,
    pub message: String,
}

impl PointsBalance {
    /// Add points
    pub fn add_points(&mut self, points: i64) {
        self.balance += points;
        if points > 0 {
            self.lifetime_earned += points;
        }
        self.updated_at = Some(Utc::now());
    }

    /// Deduct points
    pub fn deduct_points(&mut self, points: i64) -> bool {
        if self.balance >= points {
            self.balance -= points;
            self.lifetime_redeemed += points;
            self.updated_at = Some(Utc::now());
            true
        } else {
            false
        }
    }

    /// Check if can redeem
    pub fn can_redeem(&self, points: i64) -> bool {
        self.balance >= points
    }
}
