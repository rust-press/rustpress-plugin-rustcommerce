//! Inventory API Handlers
//!
//! HTTP request handlers for inventory management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct StockQuery {
    pub warehouse_id: Option<Uuid>,
    pub low_stock: Option<bool>,
    pub out_of_stock: Option<bool>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StockUpdateRequest {
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub quantity: i32,
    pub operation: String, // set, add, subtract
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkStockUpdateRequest {
    pub items: Vec<StockUpdateItem>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StockUpdateItem {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct TransferStockRequest {
    pub product_id: Uuid,
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub quantity: i32,
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StockLevelResponse {
    pub product_id: Uuid,
    pub sku: String,
    pub product_name: String,
    pub total_quantity: i32,
    pub reserved: i32,
    pub available: i32,
    pub low_stock_threshold: Option<i32>,
    pub status: String,
    pub warehouse_levels: Vec<WarehouseLevel>,
}

#[derive(Debug, Serialize)]
pub struct WarehouseLevel {
    pub warehouse_id: Uuid,
    pub warehouse_name: String,
    pub quantity: i32,
    pub reserved: i32,
}

#[derive(Debug, Serialize)]
pub struct ForecastResponse {
    pub product_id: Uuid,
    pub current_stock: i32,
    pub predicted_demand_30_days: i32,
    pub days_of_stock: Option<i32>,
    pub stockout_date: Option<String>,
    pub reorder_recommendation: Option<ReorderRecommendation>,
}

#[derive(Debug, Serialize)]
pub struct ReorderRecommendation {
    pub quantity: i32,
    pub supplier_id: Option<Uuid>,
    pub estimated_cost: String,
}
