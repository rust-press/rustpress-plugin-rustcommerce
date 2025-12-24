//! Inventory Admin Module
//!
//! Admin handlers for inventory management.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stock list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StockFilters {
    pub status: Option<String>,
    pub warehouse_id: Option<Uuid>,
    pub low_stock: Option<bool>,
    pub out_of_stock: Option<bool>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Stock item admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockItemView {
    pub id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub quantity: i32,
    pub reserved: i32,
    pub available: i32,
    pub low_stock_threshold: Option<i32>,
    pub status: String,
    pub warehouse_name: Option<String>,
}

/// Stock movement admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementView {
    pub id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub movement_type: String,
    pub quantity: i32,
    pub quantity_before: i32,
    pub quantity_after: i32,
    pub reason: Option<String>,
    pub created_at: String,
}

/// Warehouse admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseView {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub is_default: bool,
    pub is_active: bool,
    pub product_count: i32,
    pub total_stock: i32,
}

/// Supplier admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierView {
    pub id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub lead_time_days: Option<i32>,
    pub product_count: i32,
    pub is_active: bool,
}

/// Purchase order admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderView {
    pub id: Uuid,
    pub po_number: String,
    pub supplier_name: String,
    pub status: String,
    pub item_count: i32,
    pub total: Decimal,
    pub expected_date: Option<String>,
    pub created_at: String,
}

/// Stock alert admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertView {
    pub id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub alert_type: String,
    pub current_quantity: i32,
    pub threshold: i32,
    pub status: String,
    pub created_at: String,
}

/// Forecast admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastView {
    pub product_name: String,
    pub sku: String,
    pub current_stock: i32,
    pub predicted_demand: i32,
    pub days_of_stock: Option<i32>,
    pub stockout_date: Option<String>,
    pub recommended_reorder_qty: Option<i32>,
    pub confidence_level: Decimal,
}
