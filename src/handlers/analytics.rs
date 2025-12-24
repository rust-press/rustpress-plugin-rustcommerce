//! Analytics API Handlers
//!
//! HTTP request handlers for analytics and reporting.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub period: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub compare: Option<bool>,
    pub granularity: Option<String>, // day, week, month
}

#[derive(Debug, Serialize)]
pub struct DashboardOverviewResponse {
    pub total_sales: String,
    pub total_orders: i32,
    pub total_customers: i32,
    pub avg_order_value: String,
    pub sales_change: String,
    pub orders_change: String,
    pub customers_change: String,
    pub aov_change: String,
}

#[derive(Debug, Serialize)]
pub struct SalesChartResponse {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
}

#[derive(Debug, Serialize)]
pub struct ChartDataset {
    pub label: String,
    pub data: Vec<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SalesReportResponse {
    pub summary: SalesSummary,
    pub by_period: Vec<PeriodSales>,
    pub by_category: Vec<CategorySales>,
    pub by_product: Vec<ProductSales>,
}

#[derive(Debug, Serialize)]
pub struct SalesSummary {
    pub gross_sales: String,
    pub net_sales: String,
    pub discounts: String,
    pub shipping: String,
    pub taxes: String,
    pub refunds: String,
    pub order_count: i32,
    pub items_sold: i32,
}

#[derive(Debug, Serialize)]
pub struct PeriodSales {
    pub period: String,
    pub gross_sales: String,
    pub net_sales: String,
    pub orders: i32,
}

#[derive(Debug, Serialize)]
pub struct CategorySales {
    pub category_id: Uuid,
    pub category_name: String,
    pub items_sold: i32,
    pub revenue: String,
}

#[derive(Debug, Serialize)]
pub struct ProductSales {
    pub product_id: Uuid,
    pub product_name: String,
    pub sku: Option<String>,
    pub items_sold: i32,
    pub revenue: String,
}

#[derive(Debug, Serialize)]
pub struct CustomerReportResponse {
    pub total_customers: i64,
    pub new_customers: i64,
    pub returning_customers: i64,
    pub avg_customer_value: String,
    pub repeat_purchase_rate: String,
    pub top_customers: Vec<TopCustomer>,
}

#[derive(Debug, Serialize)]
pub struct TopCustomer {
    pub customer_id: Uuid,
    pub customer_name: String,
    pub customer_email: String,
    pub total_orders: i32,
    pub total_spent: String,
    pub last_order_date: String,
}

#[derive(Debug, Serialize)]
pub struct MarketingReportResponse {
    pub campaigns_sent: i64,
    pub emails_opened: i64,
    pub clicks: i64,
    pub conversions: i64,
    pub revenue_attributed: String,
    pub carts_abandoned: i64,
    pub carts_recovered: i64,
    pub recovery_revenue: String,
    pub coupon_usage: CouponUsageStats,
}

#[derive(Debug, Serialize)]
pub struct CouponUsageStats {
    pub total_uses: i64,
    pub unique_customers: i64,
    pub total_discount: String,
    pub top_coupons: Vec<TopCoupon>,
}

#[derive(Debug, Serialize)]
pub struct TopCoupon {
    pub code: String,
    pub uses: i64,
    pub discount_given: String,
}

#[derive(Debug, Serialize)]
pub struct SearchReportResponse {
    pub total_searches: i64,
    pub unique_searches: i64,
    pub zero_result_rate: String,
    pub search_to_purchase_rate: String,
    pub top_searches: Vec<SearchTerm>,
    pub zero_result_searches: Vec<SearchTerm>,
}

#[derive(Debug, Serialize)]
pub struct SearchTerm {
    pub term: String,
    pub count: i64,
    pub conversions: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub report_type: String,
    pub format: String, // csv, xlsx, pdf
    #[serde(flatten)]
    pub filters: AnalyticsQuery,
}

#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub download_url: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize)]
pub struct RealTimeStatsResponse {
    pub active_visitors: i32,
    pub orders_today: i32,
    pub revenue_today: String,
    pub active_carts: i32,
    pub recent_orders: Vec<RecentOrderInfo>,
}

#[derive(Debug, Serialize)]
pub struct RecentOrderInfo {
    pub order_number: String,
    pub customer_name: String,
    pub total: String,
    pub status: String,
    pub created_at: String,
}
