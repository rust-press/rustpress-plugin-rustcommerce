//! Advanced Analytics Models
//!
//! E-commerce analytics, reporting, and insights.
//! Self-contained analytics system with no external dependencies.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use std::collections::HashMap;

/// Analytics event (raw event tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub event_type: EventType,
    pub event_name: String,

    // Context
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub visitor_id: Option<String>,

    // Entity references
    pub product_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub cart_id: Option<Uuid>,

    // Event data
    pub value: Option<Decimal>,
    pub quantity: Option<i32>,
    pub properties: HashMap<String, String>,

    // Source tracking
    pub source: Option<String>,
    pub medium: Option<String>,
    pub campaign: Option<String>,
    pub referrer: Option<String>,

    // Device/location
    pub device_type: Option<DeviceType>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,

    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Page events
    PageView,
    ProductView,
    CategoryView,
    Search,

    // Cart events
    AddToCart,
    RemoveFromCart,
    UpdateCart,
    ViewCart,

    // Checkout events
    BeginCheckout,
    AddShippingInfo,
    AddPaymentInfo,
    Purchase,

    // User events
    SignUp,
    SignIn,
    SignOut,
    Subscribe,

    // Engagement
    WishlistAdd,
    WishlistRemove,
    ProductCompare,
    ProductReview,
    SocialShare,

    // Custom
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    Unknown,
}

/// Sales report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesReport {
    pub site_id: Option<Uuid>,
    pub period: ReportPeriod,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,

    // Totals
    pub total_sales: Decimal,
    pub total_orders: i32,
    pub total_items_sold: i32,
    pub avg_order_value: Decimal,

    // Breakdown
    pub gross_sales: Decimal,
    pub discounts: Decimal,
    pub refunds: Decimal,
    pub shipping: Decimal,
    pub taxes: Decimal,
    pub net_sales: Decimal,

    // Comparison with previous period
    pub sales_change: Decimal,
    pub orders_change: Decimal,
    pub aov_change: Decimal,

    // Daily/weekly/monthly breakdown
    pub time_series: Vec<SalesTimeSeries>,

    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportPeriod {
    Today,
    Yesterday,
    Last7Days,
    Last30Days,
    ThisMonth,
    LastMonth,
    ThisQuarter,
    LastQuarter,
    ThisYear,
    LastYear,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTimeSeries {
    pub date: NaiveDate,
    pub sales: Decimal,
    pub orders: i32,
    pub items_sold: i32,
    pub avg_order_value: Decimal,
}

/// Product analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAnalytics {
    pub product_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Views and engagement
    pub views: i64,
    pub unique_views: i64,
    pub add_to_cart_count: i32,
    pub cart_to_view_rate: Decimal,

    // Sales
    pub units_sold: i32,
    pub revenue: Decimal,
    pub conversion_rate: Decimal,

    // Inventory
    pub current_stock: i32,
    pub stock_value: Decimal,
    pub days_of_stock: Option<i32>,

    // Performance
    pub avg_rating: Option<Decimal>,
    pub review_count: i32,
    pub return_rate: Decimal,
    pub wishlist_count: i32,
}

/// Category analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryAnalytics {
    pub category_id: Uuid,
    pub category_name: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    pub product_count: i32,
    pub views: i64,
    pub revenue: Decimal,
    pub units_sold: i32,
    pub avg_order_value: Decimal,
    pub top_products: Vec<TopProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProduct {
    pub product_id: Uuid,
    pub product_name: String,
    pub revenue: Decimal,
    pub units_sold: i32,
}

/// Customer analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAnalytics {
    pub site_id: Option<Uuid>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    // Totals
    pub total_customers: i64,
    pub new_customers: i64,
    pub returning_customers: i64,

    // Segments
    pub customer_segments: Vec<CustomerSegment>,

    // Value metrics
    pub avg_customer_value: Decimal,
    pub avg_orders_per_customer: Decimal,
    pub customer_lifetime_value: Decimal,

    // Retention
    pub repeat_purchase_rate: Decimal,
    pub churn_rate: Decimal,

    // Acquisition
    pub acquisition_channels: Vec<AcquisitionChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegment {
    pub segment_type: SegmentType,
    pub count: i64,
    pub revenue: Decimal,
    pub avg_order_value: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentType {
    New,
    Returning,
    HighValue,
    AtRisk,
    Lost,
    Champions,
    Loyal,
    Potential,
    NeedAttention,
    Hibernating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionChannel {
    pub channel: String,
    pub customers: i64,
    pub revenue: Decimal,
    pub conversion_rate: Decimal,
}

/// Funnel analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelAnalytics {
    pub funnel_type: FunnelType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub steps: Vec<FunnelStep>,
    pub overall_conversion: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunnelType {
    Purchase,
    Checkout,
    Registration,
    ProductView,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelStep {
    pub step_name: String,
    pub step_order: i32,
    pub visitors: i64,
    pub conversion_rate: Decimal,
    pub drop_off_rate: Decimal,
    pub avg_time_seconds: Option<i64>,
}

/// Revenue by source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueBySource {
    pub source: String,
    pub medium: Option<String>,
    pub campaign: Option<String>,
    pub revenue: Decimal,
    pub orders: i32,
    pub conversion_rate: Decimal,
    pub avg_order_value: Decimal,
}

/// Search analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    pub total_searches: i64,
    pub unique_searches: i64,
    pub searches_with_results: i64,
    pub searches_without_results: i64,
    pub search_to_purchase_rate: Decimal,

    pub top_search_terms: Vec<SearchTerm>,
    pub zero_result_terms: Vec<SearchTerm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTerm {
    pub term: String,
    pub count: i64,
    pub results_count: Option<i64>,
    pub click_through_rate: Option<Decimal>,
    pub conversion_rate: Option<Decimal>,
}

/// Cart analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    pub carts_created: i64,
    pub carts_converted: i64,
    pub carts_abandoned: i64,
    pub abandonment_rate: Decimal,

    pub avg_cart_value: Decimal,
    pub avg_items_per_cart: Decimal,

    pub abandoned_value: Decimal,
    pub recovered_value: Decimal,
    pub recovery_rate: Decimal,

    pub abandonment_reasons: Vec<AbandonmentReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonmentReason {
    pub reason: String,
    pub count: i64,
    pub percentage: Decimal,
}

/// Real-time analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeAnalytics {
    pub timestamp: DateTime<Utc>,
    pub active_visitors: i32,
    pub active_carts: i32,
    pub orders_last_hour: i32,
    pub revenue_last_hour: Decimal,

    pub top_products_now: Vec<ProductActivity>,
    pub recent_orders: Vec<RecentOrder>,
    pub recent_signups: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductActivity {
    pub product_id: Uuid,
    pub product_name: String,
    pub views: i32,
    pub adds_to_cart: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentOrder {
    pub order_id: Uuid,
    pub total: Decimal,
    pub items_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Dashboard widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub widget_type: WidgetType,
    pub title: String,
    pub value: String,
    pub change: Option<Decimal>,
    pub change_direction: Option<ChangeDirection>,
    pub chart_data: Option<Vec<ChartDataPoint>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetType {
    TotalSales,
    OrderCount,
    AvgOrderValue,
    ConversionRate,
    NewCustomers,
    TopProducts,
    RevenueChart,
    TrafficSources,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeDirection {
    Up,
    Down,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: Decimal,
}

/// Custom report definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReport {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub report_type: CustomReportType,
    pub metrics: Vec<String>,
    pub dimensions: Vec<String>,
    pub filters: Vec<ReportFilter>,
    pub date_range: ReportDateRange,
    pub schedule: Option<ReportSchedule>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomReportType {
    Sales,
    Products,
    Customers,
    Traffic,
    Marketing,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    GreaterThan,
    LessThan,
    Between,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDateRange {
    pub range_type: ReportPeriod,
    pub custom_start: Option<DateTime<Utc>>,
    pub custom_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    pub frequency: ScheduleFrequency,
    pub recipients: Vec<String>,
    pub format: ExportFormat,
    pub next_run: Option<DateTime<Utc>>,
    pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Csv,
    Excel,
    Pdf,
    Json,
}

/// Analytics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSettings {
    pub enabled: bool,
    pub track_guests: bool,
    pub track_search: bool,
    pub track_events: bool,
    pub anonymize_ip: bool,
    pub data_retention_days: i32,
    pub exclude_admin_tracking: bool,
    pub exclude_bots: bool,
    pub custom_dimensions: Vec<CustomDimension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDimension {
    pub name: String,
    pub field: String,
    pub data_type: DimensionDataType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionDataType {
    String,
    Number,
    Boolean,
    Date,
}

impl Default for AnalyticsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            track_guests: true,
            track_search: true,
            track_events: true,
            anonymize_ip: false,
            data_retention_days: 365,
            exclude_admin_tracking: true,
            exclude_bots: true,
            custom_dimensions: Vec::new(),
        }
    }
}

/// Track event request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackEventRequest {
    pub event_type: EventType,
    pub event_name: Option<String>,
    pub product_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub value: Option<Decimal>,
    pub quantity: Option<i32>,
    pub properties: Option<HashMap<String, String>>,
}

/// Analytics query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQueryRequest {
    pub metrics: Vec<String>,
    pub dimensions: Option<Vec<String>>,
    pub filters: Option<Vec<ReportFilter>>,
    pub date_range: ReportDateRange,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order_by: Option<String>,
    pub order_direction: Option<OrderDirection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Analytics query response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQueryResponse {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub totals: Option<HashMap<String, Decimal>>,
    pub row_count: i32,
    pub query_time_ms: i64,
}
