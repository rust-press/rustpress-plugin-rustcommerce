//! Vendors API Handlers
//!
//! HTTP request handlers for multi-vendor marketplace.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct VendorQuery {
    pub status: Option<String>,
    pub verified: Option<bool>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct VendorRegistrationRequest {
    pub store_name: String,
    pub store_slug: Option<String>,
    pub description: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub business_type: Option<String>,
    pub tax_id: Option<String>,
    pub bank_account: Option<BankAccountInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BankAccountInfo {
    pub account_name: String,
    pub account_number: String,
    pub bank_name: String,
    pub routing_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VendorProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub regular_price: String,
    pub sale_price: Option<String>,
    pub sku: Option<String>,
    pub stock_quantity: Option<i32>,
    pub categories: Vec<Uuid>,
    pub images: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct VendorDashboardData {
    pub total_sales: String,
    pub pending_orders: i32,
    pub total_products: i32,
    pub average_rating: Option<String>,
    pub pending_balance: String,
    pub total_withdrawn: String,
}

#[derive(Debug, Serialize)]
pub struct VendorResponse {
    pub id: Uuid,
    pub store_name: String,
    pub store_slug: String,
    pub status: String,
    pub verified: bool,
    pub product_count: i32,
    pub total_sales: String,
}
