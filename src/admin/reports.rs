//! Admin Reports
//!
//! Reports and analytics admin functionality.

use serde::{Deserialize, Serialize};

/// Report type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportType {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
}

/// Get available report types
pub fn get_report_types() -> Vec<ReportType> {
    vec![
        ReportType {
            id: "orders".to_string(),
            title: "Orders".to_string(),
            description: "View order reports and statistics".to_string(),
            icon: "shopping-bag".to_string(),
        },
        ReportType {
            id: "customers".to_string(),
            title: "Customers".to_string(),
            description: "View customer reports and statistics".to_string(),
            icon: "users".to_string(),
        },
        ReportType {
            id: "stock".to_string(),
            title: "Stock".to_string(),
            description: "View stock status and low stock items".to_string(),
            icon: "package".to_string(),
        },
        ReportType {
            id: "taxes".to_string(),
            title: "Taxes".to_string(),
            description: "View tax reports by date and code".to_string(),
            icon: "file-text".to_string(),
        },
    ]
}

/// Report date ranges
pub fn get_date_ranges() -> Vec<(&'static str, &'static str)> {
    vec![
        ("today", "Today"),
        ("yesterday", "Yesterday"),
        ("week", "This week"),
        ("last_week", "Last week"),
        ("month", "This month"),
        ("last_month", "Last month"),
        ("quarter", "This quarter"),
        ("year", "This year"),
        ("custom", "Custom"),
    ]
}

/// Report chart types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Doughnut,
    Area,
}

/// Report chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub chart_type: ChartType,
    pub title: String,
    pub x_axis_label: String,
    pub y_axis_label: String,
    pub show_legend: bool,
}

/// Sales report columns
pub fn get_sales_report_columns() -> Vec<(&'static str, &'static str)> {
    vec![
        ("date", "Date"),
        ("orders", "Orders"),
        ("gross_sales", "Gross Sales"),
        ("refunds", "Refunds"),
        ("coupons", "Coupons"),
        ("net_sales", "Net Sales"),
        ("shipping", "Shipping"),
        ("tax", "Tax"),
        ("total", "Total"),
    ]
}

/// Customer report columns
pub fn get_customer_report_columns() -> Vec<(&'static str, &'static str)> {
    vec![
        ("customer", "Customer"),
        ("email", "Email"),
        ("orders", "Orders"),
        ("total_spent", "Total Spent"),
        ("avg_order_value", "AOV"),
        ("last_order", "Last Order"),
    ]
}

/// Stock report columns
pub fn get_stock_report_columns() -> Vec<(&'static str, &'static str)> {
    vec![
        ("product", "Product"),
        ("sku", "SKU"),
        ("status", "Stock Status"),
        ("stock", "Stock"),
        ("actions", "Actions"),
    ]
}

/// Export formats
pub fn get_export_formats() -> Vec<(&'static str, &'static str)> {
    vec![
        ("csv", "CSV"),
        ("xlsx", "Excel (XLSX)"),
        ("pdf", "PDF"),
    ]
}
