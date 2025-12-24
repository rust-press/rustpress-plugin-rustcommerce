//! RustCommerce Plugin Implementation

use async_trait::async_trait;
use rustpress_core::context::AppContext;
use rustpress_core::error::Result;
use rustpress_core::plugin::{Plugin, PluginInfo, PluginState};
use semver::Version;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, error};

use crate::settings::RustCommerceSettings;
use crate::services::*;

/// The main RustCommerce plugin
pub struct RustCommercePlugin {
    info: PluginInfo,
    state: RwLock<PluginState>,
    settings: RwLock<RustCommerceSettings>,

    // Services
    pricing_service: RwLock<Option<Arc<pricing::PricingService>>>,
    cart_service: RwLock<Option<Arc<cart::CartService>>>,
    checkout_service: RwLock<Option<Arc<checkout::CheckoutService>>>,
    inventory_service: RwLock<Option<Arc<inventory::InventoryService>>>,
    shipping_service: RwLock<Option<Arc<shipping::ShippingService>>>,
    tax_service: RwLock<Option<Arc<tax::TaxService>>>,
    order_service: RwLock<Option<Arc<order::OrderService>>>,
    customer_service: RwLock<Option<Arc<customer::CustomerService>>>,
}

impl RustCommercePlugin {
    /// Create a new RustCommerce plugin instance
    pub fn new() -> Self {
        let info = PluginInfo::new(
            crate::PLUGIN_ID,
            "RustCommerce",
            Version::parse(crate::VERSION).unwrap_or(Version::new(1, 0, 0)),
        )
        .with_description("Complete e-commerce solution for RustPress - Products, Cart, Checkout, Orders, Payments")
        .with_author("RustPress Team");

        Self {
            info,
            state: RwLock::new(PluginState::Inactive),
            settings: RwLock::new(RustCommerceSettings::default()),
            pricing_service: RwLock::new(None),
            cart_service: RwLock::new(None),
            checkout_service: RwLock::new(None),
            inventory_service: RwLock::new(None),
            shipping_service: RwLock::new(None),
            tax_service: RwLock::new(None),
            order_service: RwLock::new(None),
            customer_service: RwLock::new(None),
        }
    }

    /// Get the current settings
    pub fn settings(&self) -> RustCommerceSettings {
        self.settings.read().clone()
    }

    /// Update settings
    pub fn update_settings(&self, settings: RustCommerceSettings) {
        *self.settings.write() = settings;
    }

    /// Get pricing service
    pub fn pricing(&self) -> Option<Arc<pricing::PricingService>> {
        self.pricing_service.read().clone()
    }

    /// Get cart service
    pub fn cart(&self) -> Option<Arc<cart::CartService>> {
        self.cart_service.read().clone()
    }

    /// Get checkout service
    pub fn checkout(&self) -> Option<Arc<checkout::CheckoutService>> {
        self.checkout_service.read().clone()
    }

    /// Get inventory service
    pub fn inventory(&self) -> Option<Arc<inventory::InventoryService>> {
        self.inventory_service.read().clone()
    }

    /// Get shipping service
    pub fn shipping(&self) -> Option<Arc<shipping::ShippingService>> {
        self.shipping_service.read().clone()
    }

    /// Get tax service
    pub fn tax(&self) -> Option<Arc<tax::TaxService>> {
        self.tax_service.read().clone()
    }

    /// Get order service
    pub fn orders(&self) -> Option<Arc<order::OrderService>> {
        self.order_service.read().clone()
    }

    /// Get customer service
    pub fn customers(&self) -> Option<Arc<customer::CustomerService>> {
        self.customer_service.read().clone()
    }

    /// Initialize services
    async fn init_services(&self, ctx: &AppContext) -> Result<()> {
        let settings = self.settings();

        // Initialize pricing service
        let pricing = Arc::new(pricing::PricingService::new(settings.clone()));
        *self.pricing_service.write() = Some(pricing.clone());

        // Initialize tax service
        let tax = Arc::new(tax::TaxService::new(settings.clone()));
        *self.tax_service.write() = Some(tax.clone());

        // Initialize shipping service
        let shipping = Arc::new(shipping::ShippingService::new(settings.clone()));
        *self.shipping_service.write() = Some(shipping.clone());

        // Initialize inventory service
        let inventory = Arc::new(inventory::InventoryService::new(settings.clone()));
        *self.inventory_service.write() = Some(inventory.clone());

        // Initialize cart service
        let cart = Arc::new(cart::CartService::new(
            pricing.clone(),
            tax.clone(),
            shipping.clone(),
            settings.clone(),
        ));
        *self.cart_service.write() = Some(cart.clone());

        // Initialize order service
        let order = Arc::new(order::OrderService::new(settings.clone()));
        *self.order_service.write() = Some(order.clone());

        // Initialize customer service
        let customer = Arc::new(customer::CustomerService::new());
        *self.customer_service.write() = Some(customer.clone());

        // Initialize checkout service
        let checkout = Arc::new(checkout::CheckoutService::new(
            cart.clone(),
            order.clone(),
            inventory.clone(),
            shipping.clone(),
            tax.clone(),
            settings.clone(),
        ));
        *self.checkout_service.write() = Some(checkout);

        info!("RustCommerce services initialized");
        Ok(())
    }

    /// Register hooks
    fn register_hooks(&self, ctx: &AppContext) {
        // Register WordPress-like hooks
        // These would integrate with the HookRegistry

        debug!("Registering RustCommerce hooks");

        // Example hooks:
        // - rustcommerce_before_cart_totals
        // - rustcommerce_after_cart_totals
        // - rustcommerce_checkout_process
        // - rustcommerce_payment_complete
        // - rustcommerce_order_status_changed
        // - rustcommerce_low_stock_notification
    }

    /// Register shortcodes
    fn register_shortcodes(&self, ctx: &AppContext) {
        debug!("Registering RustCommerce shortcodes");

        // [rc_product id="123"]
        // [rc_products category="clothing" limit="10"]
        // [rc_cart]
        // [rc_checkout]
        // [rc_my_account]
        // [rc_order_tracking]
    }

    /// Register widgets
    fn register_widgets(&self, ctx: &AppContext) {
        debug!("Registering RustCommerce widgets");

        // - Mini Cart
        // - Product Categories
        // - Product Search
        // - Recent Products
        // - Featured Products
        // - On-Sale Products
        // - Best Sellers
        // - Price Filter
        // - Attribute Filter
    }

    /// Register REST API routes
    fn register_api_routes(&self, ctx: &AppContext) {
        debug!("Registering RustCommerce API routes");

        // /rc/v1/products
        // /rc/v1/products/{id}
        // /rc/v1/products/categories
        // /rc/v1/products/tags
        // /rc/v1/products/attributes
        // /rc/v1/orders
        // /rc/v1/orders/{id}
        // /rc/v1/customers
        // /rc/v1/cart
        // /rc/v1/cart/add
        // /rc/v1/cart/remove
        // /rc/v1/cart/update
        // /rc/v1/checkout
        // /rc/v1/coupons
        // /rc/v1/shipping/zones
        // /rc/v1/shipping/methods
        // /rc/v1/taxes
        // /rc/v1/reports/sales
        // /rc/v1/reports/products
        // /rc/v1/reports/customers
    }

    /// Register admin menus
    fn register_admin_menus(&self, ctx: &AppContext) {
        debug!("Registering RustCommerce admin menus");

        // Main menu: RustCommerce
        // - Dashboard
        // - Orders
        // - Products
        // - Customers
        // - Coupons
        // - Reports
        //   - Sales
        //   - Customers
        //   - Products
        //   - Stock
        // - Settings
        //   - General
        //   - Products
        //   - Tax
        //   - Shipping
        //   - Payments
        //   - Emails
        //   - Advanced
    }

    /// Run database migrations
    async fn run_migrations(&self, ctx: &AppContext) -> Result<()> {
        info!("Running RustCommerce database migrations");
        // Would run migrations from /migrations folder
        Ok(())
    }
}

impl Default for RustCommercePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for RustCommercePlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    fn state(&self) -> PluginState {
        *self.state.read()
    }

    async fn activate(&self, ctx: &AppContext) -> Result<()> {
        info!("Activating RustCommerce plugin v{}", crate::VERSION);

        // Run migrations
        self.run_migrations(ctx).await?;

        // Initialize services
        self.init_services(ctx).await?;

        // Register hooks
        self.register_hooks(ctx);

        // Register shortcodes
        self.register_shortcodes(ctx);

        // Register widgets
        self.register_widgets(ctx);

        // Register API routes
        self.register_api_routes(ctx);

        // Register admin menus
        self.register_admin_menus(ctx);

        *self.state.write() = PluginState::Active;
        info!("RustCommerce plugin activated successfully");

        Ok(())
    }

    async fn deactivate(&self, _ctx: &AppContext) -> Result<()> {
        info!("Deactivating RustCommerce plugin");

        // Clear services
        *self.pricing_service.write() = None;
        *self.cart_service.write() = None;
        *self.checkout_service.write() = None;
        *self.inventory_service.write() = None;
        *self.shipping_service.write() = None;
        *self.tax_service.write() = None;
        *self.order_service.write() = None;
        *self.customer_service.write() = None;

        *self.state.write() = PluginState::Inactive;
        info!("RustCommerce plugin deactivated");

        Ok(())
    }

    async fn on_startup(&self, ctx: &AppContext) -> Result<()> {
        debug!("RustCommerce startup");

        // Schedule cron jobs
        // - Cleanup expired carts
        // - Low stock notifications
        // - Abandoned cart emails
        // - Report generation

        Ok(())
    }

    async fn on_shutdown(&self, _ctx: &AppContext) -> Result<()> {
        debug!("RustCommerce shutdown");
        Ok(())
    }

    fn config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "title": "RustCommerce Settings",
            "properties": {
                "general": {
                    "type": "object",
                    "title": "General",
                    "properties": {
                        "store_name": { "type": "string", "title": "Store Name" },
                        "store_address": { "type": "string", "title": "Store Address" },
                        "store_city": { "type": "string", "title": "City" },
                        "store_country": { "type": "string", "title": "Country" },
                        "currency": { "type": "string", "title": "Currency", "default": "USD" }
                    }
                },
                "products": {
                    "type": "object",
                    "title": "Products",
                    "properties": {
                        "enable_reviews": { "type": "boolean", "default": true },
                        "enable_stock_management": { "type": "boolean", "default": true },
                        "low_stock_threshold": { "type": "integer", "default": 5 }
                    }
                },
                "checkout": {
                    "type": "object",
                    "title": "Checkout",
                    "properties": {
                        "enable_guest_checkout": { "type": "boolean", "default": true },
                        "enable_coupons": { "type": "boolean", "default": true }
                    }
                }
            }
        }))
    }
}
