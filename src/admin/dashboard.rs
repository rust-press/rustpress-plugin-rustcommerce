//! Admin Dashboard
//!
//! RustCommerce dashboard with stats and quick actions.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub stats: DashboardStats,
    pub recent_orders: Vec<RecentOrder>,
    pub top_sellers: Vec<TopSeller>,
    pub stock_alerts: Vec<StockAlert>,
    pub order_status_counts: OrderStatusCounts,
}

/// Dashboard statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub net_sales_today: Decimal,
    pub net_sales_today_change: Decimal,
    pub orders_today: i32,
    pub orders_today_change: Decimal,
    pub items_sold_today: i32,
    pub items_sold_today_change: Decimal,
    pub net_sales_month: Decimal,
    pub net_sales_month_change: Decimal,
    pub orders_month: i32,
    pub orders_month_change: Decimal,
}

/// Recent order for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentOrder {
    pub id: String,
    pub order_number: String,
    pub customer_name: String,
    pub status: String,
    pub total: String,
    pub date: String,
}

/// Top selling product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSeller {
    pub product_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub total: String,
}

/// Stock alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAlert {
    pub product_id: String,
    pub product_name: String,
    pub sku: Option<String>,
    pub stock: i32,
    pub alert_type: StockAlertType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StockAlertType {
    LowStock,
    OutOfStock,
}

/// Order status counts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusCounts {
    pub pending: i32,
    pub processing: i32,
    pub on_hold: i32,
    pub completed: i32,
    pub cancelled: i32,
    pub refunded: i32,
    pub failed: i32,
}

impl DashboardData {
    /// Create empty dashboard data
    pub fn empty() -> Self {
        Self {
            stats: DashboardStats {
                net_sales_today: Decimal::ZERO,
                net_sales_today_change: Decimal::ZERO,
                orders_today: 0,
                orders_today_change: Decimal::ZERO,
                items_sold_today: 0,
                items_sold_today_change: Decimal::ZERO,
                net_sales_month: Decimal::ZERO,
                net_sales_month_change: Decimal::ZERO,
                orders_month: 0,
                orders_month_change: Decimal::ZERO,
            },
            recent_orders: vec![],
            top_sellers: vec![],
            stock_alerts: vec![],
            order_status_counts: OrderStatusCounts::default(),
        }
    }
}

/// Dashboard widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: String,
    pub title: String,
    pub widget_type: WidgetType,
    pub position: i32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetType {
    SalesOverview,
    RecentOrders,
    TopSellers,
    StockAlerts,
    OrderStatus,
    SalesChart,
    RevenueChart,
    CustomerStats,
}

/// Get default dashboard widgets
pub fn get_default_widgets() -> Vec<DashboardWidget> {
    vec![
        DashboardWidget {
            id: "sales-overview".to_string(),
            title: "Sales Overview".to_string(),
            widget_type: WidgetType::SalesOverview,
            position: 1,
            enabled: true,
        },
        DashboardWidget {
            id: "sales-chart".to_string(),
            title: "Sales Chart".to_string(),
            widget_type: WidgetType::SalesChart,
            position: 2,
            enabled: true,
        },
        DashboardWidget {
            id: "order-status".to_string(),
            title: "Order Status".to_string(),
            widget_type: WidgetType::OrderStatus,
            position: 3,
            enabled: true,
        },
        DashboardWidget {
            id: "recent-orders".to_string(),
            title: "Recent Orders".to_string(),
            widget_type: WidgetType::RecentOrders,
            position: 4,
            enabled: true,
        },
        DashboardWidget {
            id: "top-sellers".to_string(),
            title: "Top Sellers".to_string(),
            widget_type: WidgetType::TopSellers,
            position: 5,
            enabled: true,
        },
        DashboardWidget {
            id: "stock-alerts".to_string(),
            title: "Stock Alerts".to_string(),
            widget_type: WidgetType::StockAlerts,
            position: 6,
            enabled: true,
        },
    ]
}
