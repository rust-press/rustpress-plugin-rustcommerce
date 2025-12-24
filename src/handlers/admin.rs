//! Admin HTTP Handlers
//!
//! HTTP request handlers for all admin endpoints.
//! These handlers serve the admin UI and handle AJAX requests.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Admin response wrapper
#[derive(Debug, Serialize)]
pub struct AdminResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Vec<String>,
}

impl<T> AdminResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            errors: vec![],
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.to_string()),
            errors: vec![message.to_string()],
        }
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl PaginationParams {
    pub fn page(&self) -> i32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> i32 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }

    pub fn offset(&self) -> i32 {
        (self.page() - 1) * self.per_page()
    }
}

/// Paginated response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

/// Bulk action request
#[derive(Debug, Deserialize)]
pub struct BulkActionRequest {
    pub action: String,
    pub ids: Vec<Uuid>,
    pub params: Option<serde_json::Value>,
}

/// Bulk action response
#[derive(Debug, Serialize)]
pub struct BulkActionResponse {
    pub success: bool,
    pub processed: i32,
    pub failed: i32,
    pub errors: Vec<BulkActionError>,
}

#[derive(Debug, Serialize)]
pub struct BulkActionError {
    pub id: Uuid,
    pub error: String,
}

// Dashboard handlers
pub mod dashboard {
    use super::*;
    use crate::admin::dashboard::{DashboardData, DashboardStats};

    /// Dashboard index page data
    #[derive(Debug, Serialize)]
    pub struct DashboardPageData {
        pub title: String,
        pub dashboard: DashboardData,
    }

    /// Get dashboard data
    pub fn get_dashboard_data() -> DashboardPageData {
        DashboardPageData {
            title: "RustCommerce Dashboard".to_string(),
            dashboard: DashboardData::empty(),
        }
    }
}

// Products handlers
pub mod products {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct ProductFilters {
        pub status: Option<String>,
        pub product_type: Option<String>,
        pub category_id: Option<Uuid>,
        pub search: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateProductRequest {
        pub name: String,
        pub slug: Option<String>,
        pub product_type: String,
        pub status: String,
        pub description: Option<String>,
        pub short_description: Option<String>,
        pub sku: Option<String>,
        pub regular_price: Option<String>,
        pub sale_price: Option<String>,
        pub manage_stock: bool,
        pub stock_quantity: Option<i32>,
        pub stock_status: String,
        pub categories: Vec<Uuid>,
        pub tags: Vec<Uuid>,
        pub images: Vec<Uuid>,
    }

    #[derive(Debug, Deserialize)]
    pub struct UpdateProductRequest {
        pub name: Option<String>,
        pub status: Option<String>,
        pub regular_price: Option<String>,
        pub sale_price: Option<String>,
        pub stock_quantity: Option<i32>,
        pub stock_status: Option<String>,
    }
}

// Orders handlers
pub mod orders {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct OrderFilters {
        pub status: Option<String>,
        pub customer_id: Option<Uuid>,
        pub date_from: Option<String>,
        pub date_to: Option<String>,
        pub search: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct UpdateOrderRequest {
        pub status: Option<String>,
        pub customer_note: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct AddOrderNoteRequest {
        pub note: String,
        pub is_customer_note: bool,
    }
}

// Customers handlers
pub mod customers {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct CustomerFilters {
        pub role: Option<String>,
        pub search: Option<String>,
        pub order_count_min: Option<i32>,
        pub total_spent_min: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }
}

// Subscriptions handlers
pub mod subscriptions {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct SubscriptionFilters {
        pub status: Option<String>,
        pub customer_id: Option<Uuid>,
        pub product_id: Option<Uuid>,
        pub next_payment_from: Option<String>,
        pub next_payment_to: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct SubscriptionActionRequest {
        pub action: String, // pause, resume, cancel, renew
        pub reason: Option<String>,
    }
}

// Bookings handlers
pub mod bookings {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct BookingFilters {
        pub status: Option<String>,
        pub resource_id: Option<Uuid>,
        pub customer_id: Option<Uuid>,
        pub date_from: Option<String>,
        pub date_to: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateBookingRequest {
        pub product_id: Uuid,
        pub resource_id: Option<Uuid>,
        pub customer_id: Uuid,
        pub start_date: String,
        pub start_time: Option<String>,
        pub end_date: Option<String>,
        pub end_time: Option<String>,
        pub persons: i32,
        pub note: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct BookingActionRequest {
        pub action: String, // confirm, cancel, complete
        pub reason: Option<String>,
        pub send_notification: bool,
    }
}

// Memberships handlers
pub mod memberships {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct MembershipFilters {
        pub status: Option<String>,
        pub plan_id: Option<Uuid>,
        pub customer_id: Option<Uuid>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateMembershipRequest {
        pub customer_id: Uuid,
        pub plan_id: Uuid,
        pub start_date: Option<String>,
        pub note: Option<String>,
    }
}

// Vendors handlers
pub mod vendors {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct VendorFilters {
        pub status: Option<String>,
        pub verified: Option<bool>,
        pub search: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct VendorActionRequest {
        pub action: String, // approve, reject, suspend, activate
        pub reason: Option<String>,
        pub send_notification: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreatePayoutRequest {
        pub vendor_id: Uuid,
        pub amount: String,
        pub payout_method: String,
        pub note: Option<String>,
    }
}

// Inventory handlers
pub mod inventory {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct StockFilters {
        pub status: Option<String>,
        pub warehouse_id: Option<Uuid>,
        pub low_stock: Option<bool>,
        pub out_of_stock: Option<bool>,
        pub search: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct StockAdjustmentRequest {
        pub product_id: Uuid,
        pub quantity: i32,
        pub adjustment_type: String, // add, subtract, set
        pub warehouse_id: Option<Uuid>,
        pub reason: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreatePurchaseOrderRequest {
        pub supplier_id: Uuid,
        pub warehouse_id: Uuid,
        pub items: Vec<PurchaseOrderItem>,
        pub expected_date: Option<String>,
        pub note: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct PurchaseOrderItem {
        pub product_id: Uuid,
        pub quantity: i32,
        pub unit_cost: String,
    }
}

// Marketing handlers
pub mod marketing {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct CouponFilters {
        pub status: Option<String>,
        pub discount_type: Option<String>,
        pub search: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateCouponRequest {
        pub code: String,
        pub discount_type: String,
        pub amount: String,
        pub description: Option<String>,
        pub usage_limit: Option<i32>,
        pub usage_limit_per_user: Option<i32>,
        pub minimum_amount: Option<String>,
        pub maximum_amount: Option<String>,
        pub expires_at: Option<String>,
        pub product_ids: Vec<Uuid>,
        pub category_ids: Vec<Uuid>,
        pub exclude_product_ids: Vec<Uuid>,
        pub exclude_category_ids: Vec<Uuid>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CampaignFilters {
        pub status: Option<String>,
        pub campaign_type: Option<String>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateCampaignRequest {
        pub name: String,
        pub campaign_type: String,
        pub subject: String,
        pub content: String,
        pub list_ids: Vec<Uuid>,
        pub scheduled_at: Option<String>,
    }
}

// Analytics handlers
pub mod analytics {
    use super::*;
    use crate::admin::analytics::DateRangeFilter;

    #[derive(Debug, Deserialize)]
    pub struct AnalyticsRequest {
        #[serde(flatten)]
        pub date_range: DateRangeFilter,
        pub compare: Option<bool>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ExportRequest {
        pub report_type: String,
        pub format: String,
        #[serde(flatten)]
        pub date_range: DateRangeFilter,
    }
}

// Invoices handlers
pub mod invoices {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct InvoiceFilters {
        pub status: Option<String>,
        pub customer_id: Option<Uuid>,
        pub date_from: Option<String>,
        pub date_to: Option<String>,
        pub overdue: Option<bool>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateInvoiceRequest {
        pub customer_id: Uuid,
        pub order_id: Option<Uuid>,
        pub due_date: Option<String>,
        pub items: Vec<InvoiceItemRequest>,
        pub notes: Option<String>,
        pub send_email: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct InvoiceItemRequest {
        pub name: String,
        pub description: Option<String>,
        pub quantity: String,
        pub unit_price: String,
        pub tax_rate: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RecordPaymentRequest {
        pub amount: String,
        pub payment_method: String,
        pub payment_date: String,
        pub transaction_id: Option<String>,
        pub notes: Option<String>,
        pub send_receipt: bool,
    }
}

// Auctions handlers
pub mod auctions {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct AuctionFilters {
        pub status: Option<String>,
        pub ending_soon: Option<bool>,
        #[serde(flatten)]
        pub pagination: PaginationParams,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreateAuctionRequest {
        pub product_id: Uuid,
        pub start_price: String,
        pub reserve_price: Option<String>,
        pub buy_now_price: Option<String>,
        pub bid_increment: String,
        pub start_date: String,
        pub end_date: String,
        pub extend_on_bid: bool,
        pub extend_minutes: Option<i32>,
    }
}

// Settings handlers
pub mod settings {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct UpdateSettingsRequest {
        pub section: String,
        pub settings: serde_json::Value,
    }

    #[derive(Debug, Serialize)]
    pub struct SettingsResponse {
        pub section: String,
        pub settings: serde_json::Value,
        pub schema: serde_json::Value,
    }
}

/// Admin route configuration
pub fn get_admin_routes() -> Vec<AdminRouteConfig> {
    vec![
        // Dashboard
        AdminRouteConfig::get("/admin/plugins/rustcommerce", "dashboard::index"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/home", "dashboard::home"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/stats", "dashboard::stats"),

        // Products
        AdminRouteConfig::get("/admin/plugins/rustcommerce/products", "products::list"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/products/new", "products::create_form"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/products", "products::create"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/products/:id", "products::edit_form"),
        AdminRouteConfig::put("/admin/plugins/rustcommerce/products/:id", "products::update"),
        AdminRouteConfig::delete("/admin/plugins/rustcommerce/products/:id", "products::delete"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/products/bulk", "products::bulk_action"),

        // Categories
        AdminRouteConfig::get("/admin/plugins/rustcommerce/products/categories", "categories::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/products/categories", "categories::create"),
        AdminRouteConfig::put("/admin/plugins/rustcommerce/products/categories/:id", "categories::update"),
        AdminRouteConfig::delete("/admin/plugins/rustcommerce/products/categories/:id", "categories::delete"),

        // Orders
        AdminRouteConfig::get("/admin/plugins/rustcommerce/orders", "orders::list"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/orders/:id", "orders::show"),
        AdminRouteConfig::put("/admin/plugins/rustcommerce/orders/:id", "orders::update"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/orders/:id/notes", "orders::add_note"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/orders/:id/refund", "orders::refund"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/orders/bulk", "orders::bulk_action"),

        // Invoices
        AdminRouteConfig::get("/admin/plugins/rustcommerce/invoices", "invoices::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/invoices", "invoices::create"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/invoices/:id", "invoices::show"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/invoices/:id/payment", "invoices::record_payment"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/invoices/:id/send", "invoices::send"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/invoices/:id/pdf", "invoices::download_pdf"),

        // Customers
        AdminRouteConfig::get("/admin/plugins/rustcommerce/customers", "customers::list"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/customers/:id", "customers::show"),
        AdminRouteConfig::put("/admin/plugins/rustcommerce/customers/:id", "customers::update"),
        AdminRouteConfig::delete("/admin/plugins/rustcommerce/customers/:id", "customers::delete"),

        // Subscriptions
        AdminRouteConfig::get("/admin/plugins/rustcommerce/subscriptions", "subscriptions::list"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/subscriptions/:id", "subscriptions::show"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/subscriptions/:id/action", "subscriptions::action"),

        // Bookings
        AdminRouteConfig::get("/admin/plugins/rustcommerce/bookings", "bookings::list"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/bookings/calendar", "bookings::calendar"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/bookings", "bookings::create"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/bookings/:id", "bookings::show"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/bookings/:id/action", "bookings::action"),

        // Resources
        AdminRouteConfig::get("/admin/plugins/rustcommerce/bookings/resources", "resources::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/bookings/resources", "resources::create"),
        AdminRouteConfig::put("/admin/plugins/rustcommerce/bookings/resources/:id", "resources::update"),

        // Memberships
        AdminRouteConfig::get("/admin/plugins/rustcommerce/subscriptions/memberships", "memberships::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/subscriptions/memberships", "memberships::create"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/subscriptions/plans", "memberships::plans"),

        // Inventory
        AdminRouteConfig::get("/admin/plugins/rustcommerce/inventory", "inventory::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/inventory/adjust", "inventory::adjust"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/inventory/movements", "inventory::movements"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/inventory/warehouses", "inventory::warehouses"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/inventory/warehouses", "inventory::create_warehouse"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/inventory/suppliers", "inventory::suppliers"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/inventory/purchase-orders", "inventory::purchase_orders"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/inventory/purchase-orders", "inventory::create_purchase_order"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/inventory/alerts", "inventory::alerts"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/inventory/forecasting", "inventory::forecasting"),

        // Vendors
        AdminRouteConfig::get("/admin/plugins/rustcommerce/vendors", "vendors::list"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/vendors/:id", "vendors::show"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/vendors/:id/action", "vendors::action"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/vendors/applications", "vendors::applications"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/vendors/payouts", "vendors::payouts"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/vendors/payouts", "vendors::create_payout"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/vendors/commissions", "vendors::commissions"),

        // Auctions
        AdminRouteConfig::get("/admin/plugins/rustcommerce/auctions", "auctions::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/auctions", "auctions::create"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/auctions/:id", "auctions::show"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/auctions/bids", "auctions::bids"),

        // Marketing - Coupons
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/coupons", "coupons::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/marketing/coupons", "coupons::create"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/coupons/:id", "coupons::show"),
        AdminRouteConfig::put("/admin/plugins/rustcommerce/marketing/coupons/:id", "coupons::update"),
        AdminRouteConfig::delete("/admin/plugins/rustcommerce/marketing/coupons/:id", "coupons::delete"),

        // Marketing - Campaigns
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/campaigns", "campaigns::list"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/marketing/campaigns", "campaigns::create"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/campaigns/:id", "campaigns::show"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/marketing/campaigns/:id/send", "campaigns::send"),

        // Marketing - Cart Recovery
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/cart-recovery", "cart_recovery::list"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/cart-recovery/stats", "cart_recovery::stats"),

        // Marketing - Rewards
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/rewards", "rewards::dashboard"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/rewards/rules", "rewards::rules"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/marketing/rewards/rules", "rewards::create_rule"),

        // Marketing - Recommendations
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/recommendations", "recommendations::widgets"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/marketing/recommendations", "recommendations::create_widget"),

        // Marketing - Dynamic Pricing
        AdminRouteConfig::get("/admin/plugins/rustcommerce/marketing/pricing", "pricing::rules"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/marketing/pricing", "pricing::create_rule"),

        // Analytics
        AdminRouteConfig::get("/admin/plugins/rustcommerce/analytics", "analytics::overview"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/analytics/sales", "analytics::sales"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/analytics/products", "analytics::products"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/analytics/customers", "analytics::customers"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/analytics/marketing", "analytics::marketing"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/analytics/search", "analytics::search"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/analytics/export", "analytics::export"),

        // Settings
        AdminRouteConfig::get("/admin/plugins/rustcommerce/settings", "settings::index"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/settings/:section", "settings::section"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/settings/:section", "settings::update"),
        AdminRouteConfig::get("/admin/plugins/rustcommerce/settings/emails", "settings::email_templates"),
        AdminRouteConfig::post("/admin/plugins/rustcommerce/settings/emails/:id", "settings::update_email_template"),
    ]
}

/// Admin route configuration
#[derive(Debug, Clone)]
pub struct AdminRouteConfig {
    pub method: String,
    pub path: String,
    pub handler: String,
}

impl AdminRouteConfig {
    pub fn get(path: &str, handler: &str) -> Self {
        Self {
            method: "GET".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        }
    }

    pub fn post(path: &str, handler: &str) -> Self {
        Self {
            method: "POST".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        }
    }

    pub fn put(path: &str, handler: &str) -> Self {
        Self {
            method: "PUT".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        }
    }

    pub fn delete(path: &str, handler: &str) -> Self {
        Self {
            method: "DELETE".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        }
    }
}
