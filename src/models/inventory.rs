//! Inventory Management and Forecasting Models
//!
//! Self-contained inventory system with stock tracking, forecasting, and alerts.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use std::collections::HashMap;

/// Inventory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub sku: String,

    // Stock levels
    pub quantity: i32,
    pub reserved_quantity: i32,
    pub available_quantity: i32,
    pub incoming_quantity: i32,

    // Thresholds
    pub low_stock_threshold: Option<i32>,
    pub out_of_stock_threshold: i32,
    pub reorder_point: Option<i32>,
    pub reorder_quantity: Option<i32>,

    // Tracking
    pub track_inventory: bool,
    pub allow_backorders: BackorderSetting,
    pub stock_status: StockStatus,

    // Location
    pub warehouse_id: Option<Uuid>,
    pub bin_location: Option<String>,

    // Costing
    pub cost_price: Option<Decimal>,
    pub cost_method: CostMethod,

    // Dates
    pub last_stocked_at: Option<DateTime<Utc>>,
    pub last_sold_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackorderSetting {
    DoNotAllow,
    AllowNotify,
    Allow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StockStatus {
    InStock,
    LowStock,
    OutOfStock,
    OnBackorder,
    Discontinued,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostMethod {
    Fifo,    // First In First Out
    Lifo,    // Last In First Out
    Average, // Weighted Average
    Specific,
}

/// Stock movement/transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub inventory_item_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub movement_type: MovementType,
    pub quantity: i32, // Positive or negative
    pub quantity_before: i32,
    pub quantity_after: i32,

    // Reference
    pub reference_type: Option<ReferenceType>,
    pub reference_id: Option<Uuid>,
    pub order_id: Option<Uuid>,

    // Details
    pub reason: Option<String>,
    pub notes: Option<String>,
    pub cost_per_unit: Option<Decimal>,
    pub total_cost: Option<Decimal>,

    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MovementType {
    // Additions
    Purchase,
    Return,
    Adjustment,
    Transfer,
    Production,
    Initial,

    // Deductions
    Sale,
    Damaged,
    Lost,
    Expired,
    WriteOff,
    Reserved,
    Unreserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceType {
    Order,
    PurchaseOrder,
    StockAdjustment,
    Transfer,
    Return,
}

/// Warehouse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub code: String,
    pub is_default: bool,
    pub is_active: bool,

    // Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,

    // Contact
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,

    // Settings
    pub fulfillment_priority: i32,
    pub pickup_enabled: bool,
    pub shipping_enabled: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Purchase order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub po_number: String,
    pub supplier_id: Uuid,
    pub warehouse_id: Uuid,
    pub status: PurchaseOrderStatus,

    // Dates
    pub order_date: DateTime<Utc>,
    pub expected_date: Option<DateTime<Utc>>,
    pub received_date: Option<DateTime<Utc>>,

    // Items
    pub items: Vec<PurchaseOrderItem>,

    // Totals
    pub subtotal: Decimal,
    pub tax: Decimal,
    pub shipping: Decimal,
    pub total: Decimal,
    pub currency: String,

    // Notes
    pub internal_notes: Option<String>,
    pub supplier_notes: Option<String>,

    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurchaseOrderStatus {
    Draft,
    Pending,
    Ordered,
    PartiallyReceived,
    Received,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderItem {
    pub id: Uuid,
    pub purchase_order_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub sku: String,
    pub product_name: String,
    pub quantity_ordered: i32,
    pub quantity_received: i32,
    pub unit_cost: Decimal,
    pub total_cost: Decimal,
}

/// Supplier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub code: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,

    // Address
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,

    // Terms
    pub payment_terms: Option<String>,
    pub lead_time_days: Option<i32>,
    pub min_order_value: Option<Decimal>,
    pub currency: Option<String>,

    // Contact
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,

    pub notes: Option<String>,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Low stock alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAlert {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub inventory_item_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub alert_type: AlertType,
    pub current_quantity: i32,
    pub threshold: i32,
    pub status: AlertStatus,
    pub notified_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    LowStock,
    OutOfStock,
    ReorderPoint,
    Overstock,
    ExpirationWarning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Snoozed,
}

/// Inventory forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryForecast {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub forecast_date: NaiveDate,

    // Current state
    pub current_stock: i32,
    pub reserved_stock: i32,
    pub incoming_stock: i32,

    // Predictions
    pub predicted_demand: i32,
    pub predicted_stock: i32,
    pub stockout_date: Option<NaiveDate>,
    pub days_of_stock: Option<i32>,

    // Recommendation
    pub recommended_reorder_date: Option<NaiveDate>,
    pub recommended_reorder_qty: Option<i32>,

    // Confidence
    pub confidence_level: Decimal,
    pub forecast_method: ForecastMethod,

    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastMethod {
    SimpleAverage,
    MovingAverage,
    ExponentialSmoothing,
    Seasonal,
    Regression,
    MachineLearning,
}

/// Sales velocity data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesVelocity {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub period_days: i32,
    pub units_sold: i32,
    pub daily_average: Decimal,
    pub weekly_average: Decimal,
    pub monthly_average: Decimal,
    pub trend: VelocityTrend,
    pub trend_percentage: Decimal,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VelocityTrend {
    Increasing,
    Stable,
    Decreasing,
}

/// Stock count/audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockCount {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
    pub count_type: CountType,
    pub status: CountStatus,
    pub name: Option<String>,

    pub items: Vec<StockCountItem>,

    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountType {
    Full,
    Partial,
    CyclicCount,
    SpotCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountStatus {
    Draft,
    InProgress,
    PendingReview,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockCountItem {
    pub id: Uuid,
    pub stock_count_id: Uuid,
    pub inventory_item_id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub expected_quantity: i32,
    pub counted_quantity: Option<i32>,
    pub variance: Option<i32>,
    pub notes: Option<String>,
    pub counted_by: Option<Uuid>,
    pub counted_at: Option<DateTime<Utc>>,
}

/// Inventory analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Stock value
    pub total_stock_value: Decimal,
    pub total_units: i64,
    pub avg_unit_value: Decimal,

    // Movement
    pub units_received: i64,
    pub units_sold: i64,
    pub units_adjusted: i64,
    pub inventory_turnover: Decimal,

    // Issues
    pub low_stock_items: i64,
    pub out_of_stock_items: i64,
    pub overstock_items: i64,

    // Accuracy
    pub count_variance_value: Decimal,
    pub count_accuracy_rate: Decimal,

    // Top items
    pub fastest_moving: Vec<MovingItemStat>,
    pub slowest_moving: Vec<MovingItemStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingItemStat {
    pub product_id: Uuid,
    pub product_name: String,
    pub units_moved: i64,
    pub stock_value: Decimal,
    pub turnover_rate: Decimal,
}

/// Inventory settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySettings {
    pub track_inventory: bool,
    pub manage_stock: bool,
    pub notify_low_stock: bool,
    pub notify_out_of_stock: bool,
    pub low_stock_threshold: i32,
    pub notification_emails: Vec<String>,
    pub hide_out_of_stock: bool,
    pub reserve_stock_on_cart: bool,
    pub cart_reservation_minutes: i32,
    pub default_backorder_setting: BackorderSetting,
    pub show_stock_quantity: bool,
    pub stock_display_format: StockDisplayFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StockDisplayFormat {
    ExactQuantity,
    InStock,
    LowStock,
    Ranges,
}

impl Default for InventorySettings {
    fn default() -> Self {
        Self {
            track_inventory: true,
            manage_stock: true,
            notify_low_stock: true,
            notify_out_of_stock: true,
            low_stock_threshold: 10,
            notification_emails: Vec::new(),
            hide_out_of_stock: false,
            reserve_stock_on_cart: true,
            cart_reservation_minutes: 15,
            default_backorder_setting: BackorderSetting::DoNotAllow,
            show_stock_quantity: false,
            stock_display_format: StockDisplayFormat::InStock,
        }
    }
}

impl InventoryItem {
    /// Calculate available quantity
    pub fn calculate_available(&self) -> i32 {
        (self.quantity - self.reserved_quantity).max(0)
    }

    /// Check if in stock
    pub fn is_in_stock(&self) -> bool {
        self.calculate_available() > self.out_of_stock_threshold
    }

    /// Check if low stock
    pub fn is_low_stock(&self) -> bool {
        if let Some(threshold) = self.low_stock_threshold {
            self.calculate_available() <= threshold && self.calculate_available() > self.out_of_stock_threshold
        } else {
            false
        }
    }

    /// Check if needs reorder
    pub fn needs_reorder(&self) -> bool {
        if let Some(reorder_point) = self.reorder_point {
            self.calculate_available() <= reorder_point
        } else {
            false
        }
    }

    /// Get current stock status
    pub fn get_stock_status(&self) -> StockStatus {
        let available = self.calculate_available();
        if available <= self.out_of_stock_threshold {
            StockStatus::OutOfStock
        } else if self.is_low_stock() {
            StockStatus::LowStock
        } else {
            StockStatus::InStock
        }
    }
}

/// Adjust stock request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustStockRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: i32, // Can be positive or negative
    pub movement_type: MovementType,
    pub reason: Option<String>,
    pub notes: Option<String>,
}

/// Transfer stock request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferStockRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub quantity: i32,
    pub notes: Option<String>,
}
