//! Analytics Admin Module
//!
//! Admin handlers for analytics and reporting.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeFilter {
    pub period: String, // today, yesterday, last_7_days, last_30_days, this_month, last_month, custom
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl Default for DateRangeFilter {
    fn default() -> Self {
        Self {
            period: "last_30_days".to_string(),
            start_date: None,
            end_date: None,
        }
    }
}

/// Dashboard overview data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOverview {
    pub total_sales: Decimal,
    pub total_orders: i32,
    pub total_customers: i32,
    pub avg_order_value: Decimal,
    pub sales_change: Decimal,
    pub orders_change: Decimal,
    pub customers_change: Decimal,
    pub aov_change: Decimal,
}

/// Sales chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesChartData {
    pub labels: Vec<String>,
    pub sales: Vec<Decimal>,
    pub orders: Vec<i32>,
}

/// Top product data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProductData {
    pub product_name: String,
    pub product_image: Option<String>,
    pub units_sold: i32,
    pub revenue: Decimal,
}

/// Top category data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCategoryData {
    pub category_name: String,
    pub product_count: i32,
    pub revenue: Decimal,
}

/// Top customer data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCustomerData {
    pub customer_name: String,
    pub customer_email: String,
    pub total_orders: i32,
    pub total_spent: Decimal,
}

/// Sales report data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesReportData {
    pub gross_sales: Decimal,
    pub net_sales: Decimal,
    pub discounts: Decimal,
    pub shipping: Decimal,
    pub taxes: Decimal,
    pub refunds: Decimal,
    pub order_count: i32,
    pub items_sold: i32,
    pub avg_order_value: Decimal,
    pub daily_data: Vec<DailySalesData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySalesData {
    pub date: String,
    pub gross_sales: Decimal,
    pub net_sales: Decimal,
    pub orders: i32,
}

/// Product report data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReportData {
    pub total_products: i32,
    pub total_views: i64,
    pub total_sold: i32,
    pub total_revenue: Decimal,
    pub products: Vec<ProductPerformance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPerformance {
    pub product_name: String,
    pub sku: Option<String>,
    pub views: i64,
    pub add_to_cart: i32,
    pub units_sold: i32,
    pub revenue: Decimal,
    pub conversion_rate: Decimal,
}

/// Customer report data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerReportData {
    pub total_customers: i64,
    pub new_customers: i64,
    pub returning_customers: i64,
    pub avg_customer_value: Decimal,
    pub repeat_purchase_rate: Decimal,
    pub customer_segments: Vec<CustomerSegmentData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegmentData {
    pub segment: String,
    pub count: i64,
    pub revenue: Decimal,
}

/// Marketing report data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingReportData {
    pub campaigns_sent: i64,
    pub emails_opened: i64,
    pub clicks: i64,
    pub conversions: i64,
    pub revenue_attributed: Decimal,
    pub carts_abandoned: i64,
    pub carts_recovered: i64,
    pub recovery_revenue: Decimal,
}

/// Search report data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchReportData {
    pub total_searches: i64,
    pub unique_searches: i64,
    pub zero_result_rate: Decimal,
    pub search_to_purchase_rate: Decimal,
    pub top_searches: Vec<SearchTermData>,
    pub zero_result_searches: Vec<SearchTermData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTermData {
    pub term: String,
    pub count: i64,
    pub conversion_rate: Option<Decimal>,
}

/// Export options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub report_type: String,
    pub format: String, // csv, excel, pdf
    pub date_range: DateRangeFilter,
    pub include_charts: bool,
}

/// Real-time stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeStats {
    pub active_visitors: i32,
    pub orders_today: i32,
    pub revenue_today: Decimal,
    pub active_carts: i32,
    pub recent_orders: Vec<RecentOrderData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentOrderData {
    pub order_number: String,
    pub customer_name: String,
    pub total: Decimal,
    pub status: String,
    pub created_at: String,
}
