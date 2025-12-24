//! Multi-vendor Marketplace Models
//!
//! Vendor/seller management for marketplace functionality.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Vendor/Seller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vendor {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub user_id: Uuid,
    pub store_name: String,
    pub store_slug: String,
    pub store_description: Option<String>,
    pub store_logo: Option<String>,
    pub store_banner: Option<String>,
    pub status: VendorStatus,

    // Contact
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<VendorAddress>,

    // Settings
    pub commission_type: CommissionType,
    pub commission_rate: Decimal,
    pub commission_fixed: Option<Decimal>,
    pub payment_method: PayoutMethod,
    pub payment_schedule: PayoutSchedule,
    pub min_payout_amount: Decimal,

    // Bank/Payment details
    pub bank_details: Option<BankDetails>,
    pub paypal_email: Option<String>,
    pub stripe_connect_id: Option<String>,

    // Statistics
    pub total_sales: Decimal,
    pub total_orders: i32,
    pub product_count: i32,
    pub average_rating: Option<Decimal>,
    pub review_count: i32,

    // Verification
    pub verified: bool,
    pub featured: bool,
    pub trusted: bool,

    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VendorStatus {
    Pending,
    Active,
    Suspended,
    Vacation,
    Closed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommissionType {
    Percentage,
    Fixed,
    Combined,  // Percentage + fixed fee
    Tiered,    // Different rates based on volume
    Category,  // Different rates per category
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethod {
    BankTransfer,
    PayPal,
    Stripe,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutSchedule {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Manual,
}

/// Vendor address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorAddress {
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

/// Bank details for payouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDetails {
    pub account_name: String,
    pub account_number: String,
    pub routing_number: Option<String>,
    pub bank_name: String,
    pub bank_address: Option<String>,
    pub swift_code: Option<String>,
    pub iban: Option<String>,
}

/// Vendor commission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorCommission {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub order_id: Uuid,
    pub order_item_id: Uuid,
    pub product_id: Uuid,
    pub order_total: Decimal,
    pub commission_amount: Decimal,
    pub vendor_earning: Decimal,
    pub commission_rate: Decimal,
    pub status: CommissionStatus,
    pub payout_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommissionStatus {
    Pending,
    Processing,
    Paid,
    Refunded,
    Cancelled,
    OnHold,
}

/// Vendor payout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorPayout {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub amount: Decimal,
    pub commission_ids: Vec<Uuid>,
    pub payout_method: PayoutMethod,
    pub status: PayoutStatus,
    pub transaction_id: Option<String>,
    pub notes: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Pending,
    Scheduled,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Vendor order (view of order for vendor)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorOrder {
    pub id: Uuid,
    pub order_id: Uuid,
    pub vendor_id: Uuid,
    pub order_status: String,
    pub items: Vec<VendorOrderItem>,
    pub subtotal: Decimal,
    pub shipping_cost: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
    pub commission: Decimal,
    pub earning: Decimal,

    // Customer info (limited)
    pub customer_name: String,
    pub shipping_address: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorOrderItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub variation_id: Option<Uuid>,
    pub quantity: i32,
    pub price: Decimal,
    pub total: Decimal,
}

/// Vendor shipping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorShipping {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub shipping_zone_id: Uuid,
    pub method_type: VendorShippingMethod,
    pub title: String,
    pub cost: Decimal,
    pub cost_per_item: Option<Decimal>,
    pub free_shipping_threshold: Option<Decimal>,
    pub processing_time: Option<String>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VendorShippingMethod {
    FlatRate,
    FreeShipping,
    LocalPickup,
    TableRate,
    VendorCalculated,
}

/// Vendor review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorReview {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub customer_id: Uuid,
    pub order_id: Option<Uuid>,
    pub rating: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: ReviewStatus,
    pub is_verified_purchase: bool,
    pub helpful_votes: i32,
    pub unhelpful_votes: i32,
    pub vendor_response: Option<String>,
    pub vendor_response_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Pending,
    Approved,
    Spam,
    Trash,
}

/// Vendor coupon (vendor-specific discount)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorCoupon {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: VendorDiscountType,
    pub amount: Decimal,
    pub min_purchase: Option<Decimal>,
    pub max_discount: Option<Decimal>,
    pub product_ids: Option<Vec<Uuid>>,
    pub exclude_product_ids: Option<Vec<Uuid>>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub usage_limit_per_user: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VendorDiscountType {
    Percentage,
    FixedCart,
    FixedProduct,
}

/// Vendor application (to become a vendor)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorApplication {
    pub id: Uuid,
    pub user_id: Uuid,
    pub store_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub business_type: Option<String>,
    pub description: String,
    pub product_types: Option<String>,
    pub expected_products: Option<i32>,
    pub website: Option<String>,
    pub social_media: HashMap<String, String>,
    pub documents: Vec<String>,
    pub status: ApplicationStatus,
    pub admin_notes: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
    MoreInfoNeeded,
}

/// Vendor dashboard statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorDashboardStats {
    pub vendor_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Sales
    pub total_sales: Decimal,
    pub order_count: i32,
    pub avg_order_value: Decimal,

    // Products
    pub product_count: i32,
    pub low_stock_count: i32,
    pub out_of_stock_count: i32,

    // Earnings
    pub gross_earnings: Decimal,
    pub commission_paid: Decimal,
    pub net_earnings: Decimal,
    pub pending_earnings: Decimal,

    // Ratings
    pub average_rating: Decimal,
    pub new_reviews: i32,

    // Comparison
    pub sales_change_percent: Decimal,
    pub orders_change_percent: Decimal,
}

/// Marketplace settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSettings {
    pub enabled: bool,
    pub registration_enabled: bool,
    pub require_approval: bool,
    pub default_commission_type: CommissionType,
    pub default_commission_rate: Decimal,
    pub default_payout_schedule: PayoutSchedule,
    pub min_payout_amount: Decimal,
    pub vendor_can_set_shipping: bool,
    pub vendor_can_create_coupons: bool,
    pub vendor_dashboard_enabled: bool,
    pub seller_verification_required: bool,
    pub show_vendor_info_on_product: bool,
    pub show_vendor_rating: bool,
}

impl Default for MarketplaceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            registration_enabled: true,
            require_approval: true,
            default_commission_type: CommissionType::Percentage,
            default_commission_rate: Decimal::from(10),
            default_payout_schedule: PayoutSchedule::Monthly,
            min_payout_amount: Decimal::from(50),
            vendor_can_set_shipping: true,
            vendor_can_create_coupons: true,
            vendor_dashboard_enabled: true,
            seller_verification_required: false,
            show_vendor_info_on_product: true,
            show_vendor_rating: true,
        }
    }
}

impl Vendor {
    /// Check if vendor is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, VendorStatus::Active)
    }

    /// Check if vendor can sell
    pub fn can_sell(&self) -> bool {
        matches!(self.status, VendorStatus::Active | VendorStatus::Vacation)
    }

    /// Calculate commission for an amount
    pub fn calculate_commission(&self, amount: Decimal) -> Decimal {
        match self.commission_type {
            CommissionType::Percentage => {
                amount * self.commission_rate / Decimal::from(100)
            }
            CommissionType::Fixed => {
                self.commission_fixed.unwrap_or(Decimal::ZERO)
            }
            CommissionType::Combined => {
                let percentage = amount * self.commission_rate / Decimal::from(100);
                percentage + self.commission_fixed.unwrap_or(Decimal::ZERO)
            }
            _ => amount * self.commission_rate / Decimal::from(100),
        }
    }

    /// Calculate vendor earning
    pub fn calculate_earning(&self, amount: Decimal) -> Decimal {
        amount - self.calculate_commission(amount)
    }

    /// Check if payout threshold is met
    pub fn payout_threshold_met(&self, pending_amount: Decimal) -> bool {
        pending_amount >= self.min_payout_amount
    }
}

/// Vendor registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorRegistrationRequest {
    pub store_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub address: Option<VendorAddress>,
}

/// Vendor update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorUpdateRequest {
    pub store_name: Option<String>,
    pub store_description: Option<String>,
    pub store_logo: Option<String>,
    pub store_banner: Option<String>,
    pub phone: Option<String>,
    pub address: Option<VendorAddress>,
}
