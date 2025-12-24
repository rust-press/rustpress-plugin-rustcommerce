//! Vendors Admin Module
//!
//! Admin handlers for multi-vendor marketplace.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Vendor list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VendorFilters {
    pub status: Option<String>,
    pub verified: Option<bool>,
    pub featured: Option<bool>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Vendor admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorView {
    pub id: Uuid,
    pub store_name: String,
    pub owner_name: String,
    pub email: String,
    pub status: String,
    pub verified: bool,
    pub featured: bool,
    pub product_count: i32,
    pub total_sales: Decimal,
    pub average_rating: Option<Decimal>,
    pub created_at: String,
}

/// Vendor application admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub id: Uuid,
    pub store_name: String,
    pub applicant_name: String,
    pub email: String,
    pub status: String,
    pub business_type: Option<String>,
    pub created_at: String,
}

/// Vendor payout admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutView {
    pub id: Uuid,
    pub vendor_name: String,
    pub amount: Decimal,
    pub payout_method: String,
    pub status: String,
    pub commission_count: i32,
    pub scheduled_date: Option<String>,
    pub processed_at: Option<String>,
}

/// Commission admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionView {
    pub id: Uuid,
    pub vendor_name: String,
    pub order_number: String,
    pub order_total: Decimal,
    pub commission_amount: Decimal,
    pub vendor_earning: Decimal,
    pub status: String,
    pub created_at: String,
}

/// Vendor dashboard stats for admin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorDashboardStats {
    pub total_vendors: i64,
    pub active_vendors: i64,
    pub pending_applications: i64,
    pub pending_payouts: i64,
    pub total_commissions: Decimal,
    pub total_payouts: Decimal,
}
