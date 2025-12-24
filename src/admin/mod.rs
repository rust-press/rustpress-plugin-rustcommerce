//! RustCommerce Admin Module
//!
//! Comprehensive admin interface for RustCommerce e-commerce plugin.
//! All menus are organized under the Plugins menu as per RustPress standards.

pub mod dashboard;
pub mod products;
pub mod orders;
pub mod customers;
pub mod settings;
pub mod reports;

// Additional admin modules for enhanced features
pub mod subscriptions;
pub mod bookings;
pub mod memberships;
pub mod rewards;
pub mod marketing;
pub mod inventory;
pub mod vendors;
pub mod analytics;
pub mod invoices;

use serde::{Deserialize, Serialize};

/// Admin menu configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMenu {
    pub items: Vec<AdminMenuItem>,
}

/// Admin menu item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminMenuItem {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub url: String,
    pub capability: String,
    pub position: i32,
    pub badge: Option<MenuBadge>,
    pub children: Vec<AdminSubmenuItem>,
}

/// Admin submenu item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSubmenuItem {
    pub id: String,
    pub title: String,
    pub url: String,
    pub capability: String,
    pub badge: Option<MenuBadge>,
}

/// Menu badge for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuBadge {
    pub count: i32,
    pub color: String,
}

impl AdminMenu {
    /// Create the complete RustCommerce admin menu
    /// All menus go under Plugins > RustCommerce
    pub fn new() -> Self {
        Self {
            items: vec![
                // Main RustCommerce menu (under Plugins)
                Self::main_menu(),
                Self::products_menu(),
                Self::orders_menu(),
                Self::customers_menu(),
                Self::marketing_menu(),
                Self::subscriptions_menu(),
                Self::bookings_menu(),
                Self::inventory_menu(),
                Self::analytics_menu(),
                Self::settings_menu(),
            ],
        }
    }

    /// Main dashboard menu
    fn main_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rustcommerce".to_string(),
            title: "RustCommerce".to_string(),
            icon: "shopping-cart".to_string(),
            url: "/admin/plugins/rustcommerce".to_string(),
            capability: "manage_rustcommerce".to_string(),
            position: 1,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-dashboard".to_string(),
                    title: "Dashboard".to_string(),
                    url: "/admin/plugins/rustcommerce".to_string(),
                    capability: "manage_rustcommerce".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-home".to_string(),
                    title: "Home".to_string(),
                    url: "/admin/plugins/rustcommerce/home".to_string(),
                    capability: "manage_rustcommerce".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Products menu with sub-menus
    fn products_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-products".to_string(),
            title: "Products".to_string(),
            icon: "package".to_string(),
            url: "/admin/plugins/rustcommerce/products".to_string(),
            capability: "manage_rc_products".to_string(),
            position: 2,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-products-all".to_string(),
                    title: "All Products".to_string(),
                    url: "/admin/plugins/rustcommerce/products".to_string(),
                    capability: "manage_rc_products".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-products-add".to_string(),
                    title: "Add New".to_string(),
                    url: "/admin/plugins/rustcommerce/products/new".to_string(),
                    capability: "edit_rc_products".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-categories".to_string(),
                    title: "Categories".to_string(),
                    url: "/admin/plugins/rustcommerce/products/categories".to_string(),
                    capability: "manage_rc_categories".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-tags".to_string(),
                    title: "Tags".to_string(),
                    url: "/admin/plugins/rustcommerce/products/tags".to_string(),
                    capability: "manage_rc_tags".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-attributes".to_string(),
                    title: "Attributes".to_string(),
                    url: "/admin/plugins/rustcommerce/products/attributes".to_string(),
                    capability: "manage_rc_attributes".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-bundles".to_string(),
                    title: "Bundles".to_string(),
                    url: "/admin/plugins/rustcommerce/products/bundles".to_string(),
                    capability: "manage_rc_products".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-reviews".to_string(),
                    title: "Reviews".to_string(),
                    url: "/admin/plugins/rustcommerce/products/reviews".to_string(),
                    capability: "manage_rc_products".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Orders menu with sub-menus
    fn orders_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-orders".to_string(),
            title: "Orders".to_string(),
            icon: "clipboard-list".to_string(),
            url: "/admin/plugins/rustcommerce/orders".to_string(),
            capability: "manage_rc_orders".to_string(),
            position: 3,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-orders-all".to_string(),
                    title: "All Orders".to_string(),
                    url: "/admin/plugins/rustcommerce/orders".to_string(),
                    capability: "manage_rc_orders".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-orders-pending".to_string(),
                    title: "Pending".to_string(),
                    url: "/admin/plugins/rustcommerce/orders?status=pending".to_string(),
                    capability: "manage_rc_orders".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-orders-processing".to_string(),
                    title: "Processing".to_string(),
                    url: "/admin/plugins/rustcommerce/orders?status=processing".to_string(),
                    capability: "manage_rc_orders".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-invoices".to_string(),
                    title: "Invoices".to_string(),
                    url: "/admin/plugins/rustcommerce/invoices".to_string(),
                    capability: "manage_rc_orders".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-refunds".to_string(),
                    title: "Refunds".to_string(),
                    url: "/admin/plugins/rustcommerce/orders/refunds".to_string(),
                    capability: "manage_rc_orders".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Customers menu
    fn customers_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-customers".to_string(),
            title: "Customers".to_string(),
            icon: "users".to_string(),
            url: "/admin/plugins/rustcommerce/customers".to_string(),
            capability: "manage_rc_customers".to_string(),
            position: 4,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-customers-all".to_string(),
                    title: "All Customers".to_string(),
                    url: "/admin/plugins/rustcommerce/customers".to_string(),
                    capability: "manage_rc_customers".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-customers-groups".to_string(),
                    title: "Customer Groups".to_string(),
                    url: "/admin/plugins/rustcommerce/customers/groups".to_string(),
                    capability: "manage_rc_customers".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-wishlists".to_string(),
                    title: "Wishlists".to_string(),
                    url: "/admin/plugins/rustcommerce/customers/wishlists".to_string(),
                    capability: "manage_rc_customers".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-waitlists".to_string(),
                    title: "Waitlists".to_string(),
                    url: "/admin/plugins/rustcommerce/customers/waitlists".to_string(),
                    capability: "manage_rc_customers".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Marketing menu
    fn marketing_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-marketing".to_string(),
            title: "Marketing".to_string(),
            icon: "megaphone".to_string(),
            url: "/admin/plugins/rustcommerce/marketing".to_string(),
            capability: "manage_rc_marketing".to_string(),
            position: 5,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-coupons".to_string(),
                    title: "Coupons".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/coupons".to_string(),
                    capability: "manage_rc_coupons".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-campaigns".to_string(),
                    title: "Email Campaigns".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/campaigns".to_string(),
                    capability: "manage_rc_marketing".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-automations".to_string(),
                    title: "Automations".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/automations".to_string(),
                    capability: "manage_rc_marketing".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-cart-recovery".to_string(),
                    title: "Cart Recovery".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/cart-recovery".to_string(),
                    capability: "manage_rc_marketing".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-recommendations".to_string(),
                    title: "Recommendations".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/recommendations".to_string(),
                    capability: "manage_rc_marketing".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-dynamic-pricing".to_string(),
                    title: "Dynamic Pricing".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/pricing".to_string(),
                    capability: "manage_rc_marketing".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-rewards".to_string(),
                    title: "Rewards Program".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/rewards".to_string(),
                    capability: "manage_rc_marketing".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-subscribers".to_string(),
                    title: "Subscribers".to_string(),
                    url: "/admin/plugins/rustcommerce/marketing/subscribers".to_string(),
                    capability: "manage_rc_marketing".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Subscriptions menu
    fn subscriptions_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-subscriptions".to_string(),
            title: "Subscriptions".to_string(),
            icon: "repeat".to_string(),
            url: "/admin/plugins/rustcommerce/subscriptions".to_string(),
            capability: "manage_rc_subscriptions".to_string(),
            position: 6,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-subscriptions-all".to_string(),
                    title: "All Subscriptions".to_string(),
                    url: "/admin/plugins/rustcommerce/subscriptions".to_string(),
                    capability: "manage_rc_subscriptions".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-memberships".to_string(),
                    title: "Memberships".to_string(),
                    url: "/admin/plugins/rustcommerce/subscriptions/memberships".to_string(),
                    capability: "manage_rc_subscriptions".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-membership-plans".to_string(),
                    title: "Membership Plans".to_string(),
                    url: "/admin/plugins/rustcommerce/subscriptions/plans".to_string(),
                    capability: "manage_rc_subscriptions".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-renewals".to_string(),
                    title: "Renewals".to_string(),
                    url: "/admin/plugins/rustcommerce/subscriptions/renewals".to_string(),
                    capability: "manage_rc_subscriptions".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Bookings menu
    fn bookings_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-bookings".to_string(),
            title: "Bookings".to_string(),
            icon: "calendar".to_string(),
            url: "/admin/plugins/rustcommerce/bookings".to_string(),
            capability: "manage_rc_bookings".to_string(),
            position: 7,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-bookings-all".to_string(),
                    title: "All Bookings".to_string(),
                    url: "/admin/plugins/rustcommerce/bookings".to_string(),
                    capability: "manage_rc_bookings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-bookings-calendar".to_string(),
                    title: "Calendar".to_string(),
                    url: "/admin/plugins/rustcommerce/bookings/calendar".to_string(),
                    capability: "manage_rc_bookings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-resources".to_string(),
                    title: "Resources".to_string(),
                    url: "/admin/plugins/rustcommerce/bookings/resources".to_string(),
                    capability: "manage_rc_bookings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-availability".to_string(),
                    title: "Availability".to_string(),
                    url: "/admin/plugins/rustcommerce/bookings/availability".to_string(),
                    capability: "manage_rc_bookings".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Inventory menu
    fn inventory_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-inventory".to_string(),
            title: "Inventory".to_string(),
            icon: "warehouse".to_string(),
            url: "/admin/plugins/rustcommerce/inventory".to_string(),
            capability: "manage_rc_inventory".to_string(),
            position: 8,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-inventory-stock".to_string(),
                    title: "Stock Levels".to_string(),
                    url: "/admin/plugins/rustcommerce/inventory".to_string(),
                    capability: "manage_rc_inventory".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-inventory-movements".to_string(),
                    title: "Stock Movements".to_string(),
                    url: "/admin/plugins/rustcommerce/inventory/movements".to_string(),
                    capability: "manage_rc_inventory".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-warehouses".to_string(),
                    title: "Warehouses".to_string(),
                    url: "/admin/plugins/rustcommerce/inventory/warehouses".to_string(),
                    capability: "manage_rc_inventory".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-suppliers".to_string(),
                    title: "Suppliers".to_string(),
                    url: "/admin/plugins/rustcommerce/inventory/suppliers".to_string(),
                    capability: "manage_rc_inventory".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-purchase-orders".to_string(),
                    title: "Purchase Orders".to_string(),
                    url: "/admin/plugins/rustcommerce/inventory/purchase-orders".to_string(),
                    capability: "manage_rc_inventory".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-stock-alerts".to_string(),
                    title: "Stock Alerts".to_string(),
                    url: "/admin/plugins/rustcommerce/inventory/alerts".to_string(),
                    capability: "manage_rc_inventory".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-forecasting".to_string(),
                    title: "Forecasting".to_string(),
                    url: "/admin/plugins/rustcommerce/inventory/forecasting".to_string(),
                    capability: "manage_rc_inventory".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Analytics menu
    fn analytics_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-analytics".to_string(),
            title: "Analytics".to_string(),
            icon: "bar-chart-2".to_string(),
            url: "/admin/plugins/rustcommerce/analytics".to_string(),
            capability: "view_rc_reports".to_string(),
            position: 9,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-analytics-overview".to_string(),
                    title: "Overview".to_string(),
                    url: "/admin/plugins/rustcommerce/analytics".to_string(),
                    capability: "view_rc_reports".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-reports-sales".to_string(),
                    title: "Sales Reports".to_string(),
                    url: "/admin/plugins/rustcommerce/analytics/sales".to_string(),
                    capability: "view_rc_reports".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-reports-products".to_string(),
                    title: "Product Reports".to_string(),
                    url: "/admin/plugins/rustcommerce/analytics/products".to_string(),
                    capability: "view_rc_reports".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-reports-customers".to_string(),
                    title: "Customer Reports".to_string(),
                    url: "/admin/plugins/rustcommerce/analytics/customers".to_string(),
                    capability: "view_rc_reports".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-reports-marketing".to_string(),
                    title: "Marketing Reports".to_string(),
                    url: "/admin/plugins/rustcommerce/analytics/marketing".to_string(),
                    capability: "view_rc_reports".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-reports-search".to_string(),
                    title: "Search Analytics".to_string(),
                    url: "/admin/plugins/rustcommerce/analytics/search".to_string(),
                    capability: "view_rc_reports".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-reports-export".to_string(),
                    title: "Export".to_string(),
                    url: "/admin/plugins/rustcommerce/analytics/export".to_string(),
                    capability: "view_rc_reports".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Settings menu
    fn settings_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-settings".to_string(),
            title: "Settings".to_string(),
            icon: "settings".to_string(),
            url: "/admin/plugins/rustcommerce/settings".to_string(),
            capability: "manage_rc_settings".to_string(),
            position: 10,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-settings-general".to_string(),
                    title: "General".to_string(),
                    url: "/admin/plugins/rustcommerce/settings".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-payments".to_string(),
                    title: "Payments".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/payments".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-shipping".to_string(),
                    title: "Shipping".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/shipping".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-tax".to_string(),
                    title: "Tax".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/tax".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-currency".to_string(),
                    title: "Currency".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/currency".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-emails".to_string(),
                    title: "Email Templates".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/emails".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-checkout".to_string(),
                    title: "Checkout".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/checkout".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-accounts".to_string(),
                    title: "Accounts".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/accounts".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-settings-advanced".to_string(),
                    title: "Advanced".to_string(),
                    url: "/admin/plugins/rustcommerce/settings/advanced".to_string(),
                    capability: "manage_rc_settings".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Get vendor menu (for marketplace)
    pub fn vendor_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-vendors".to_string(),
            title: "Vendors".to_string(),
            icon: "store".to_string(),
            url: "/admin/plugins/rustcommerce/vendors".to_string(),
            capability: "manage_rc_vendors".to_string(),
            position: 11,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-vendors-all".to_string(),
                    title: "All Vendors".to_string(),
                    url: "/admin/plugins/rustcommerce/vendors".to_string(),
                    capability: "manage_rc_vendors".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-vendors-applications".to_string(),
                    title: "Applications".to_string(),
                    url: "/admin/plugins/rustcommerce/vendors/applications".to_string(),
                    capability: "manage_rc_vendors".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-vendors-payouts".to_string(),
                    title: "Payouts".to_string(),
                    url: "/admin/plugins/rustcommerce/vendors/payouts".to_string(),
                    capability: "manage_rc_vendors".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-vendors-commissions".to_string(),
                    title: "Commissions".to_string(),
                    url: "/admin/plugins/rustcommerce/vendors/commissions".to_string(),
                    capability: "manage_rc_vendors".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Get auction menu
    pub fn auction_menu() -> AdminMenuItem {
        AdminMenuItem {
            id: "rc-auctions".to_string(),
            title: "Auctions".to_string(),
            icon: "gavel".to_string(),
            url: "/admin/plugins/rustcommerce/auctions".to_string(),
            capability: "manage_rc_auctions".to_string(),
            position: 12,
            badge: None,
            children: vec![
                AdminSubmenuItem {
                    id: "rc-auctions-all".to_string(),
                    title: "All Auctions".to_string(),
                    url: "/admin/plugins/rustcommerce/auctions".to_string(),
                    capability: "manage_rc_auctions".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-auctions-bids".to_string(),
                    title: "Bids".to_string(),
                    url: "/admin/plugins/rustcommerce/auctions/bids".to_string(),
                    capability: "manage_rc_auctions".to_string(),
                    badge: None,
                },
                AdminSubmenuItem {
                    id: "rc-auctions-settings".to_string(),
                    title: "Auction Settings".to_string(),
                    url: "/admin/plugins/rustcommerce/auctions/settings".to_string(),
                    capability: "manage_rc_auctions".to_string(),
                    badge: None,
                },
            ],
        }
    }

    /// Add optional menu items based on enabled features
    pub fn with_feature_menus(mut self, features: &EnabledFeatures) -> Self {
        if features.marketplace {
            self.items.push(Self::vendor_menu());
        }
        if features.auctions {
            self.items.push(Self::auction_menu());
        }
        self
    }
}

/// Enabled features configuration
#[derive(Debug, Clone, Default)]
pub struct EnabledFeatures {
    pub subscriptions: bool,
    pub bookings: bool,
    pub memberships: bool,
    pub marketplace: bool,
    pub auctions: bool,
    pub analytics: bool,
}

impl Default for AdminMenu {
    fn default() -> Self {
        Self::new()
    }
}

/// Admin capabilities - comprehensive list for all features
pub fn get_capabilities() -> Vec<(&'static str, &'static str)> {
    vec![
        // Core
        ("manage_rustcommerce", "Manage RustCommerce"),
        ("manage_rc_products", "Manage Products"),
        ("edit_rc_products", "Edit Products"),
        ("delete_rc_products", "Delete Products"),
        ("manage_rc_orders", "Manage Orders"),
        ("edit_rc_orders", "Edit Orders"),
        ("manage_rc_customers", "Manage Customers"),
        ("manage_rc_coupons", "Manage Coupons"),
        ("manage_rc_settings", "Manage Settings"),
        ("view_rc_reports", "View Reports"),
        ("manage_rc_categories", "Manage Categories"),
        ("manage_rc_tags", "Manage Tags"),
        ("manage_rc_attributes", "Manage Attributes"),

        // Marketing
        ("manage_rc_marketing", "Manage Marketing"),

        // Subscriptions & Memberships
        ("manage_rc_subscriptions", "Manage Subscriptions"),
        ("manage_rc_memberships", "Manage Memberships"),

        // Bookings
        ("manage_rc_bookings", "Manage Bookings"),

        // Inventory
        ("manage_rc_inventory", "Manage Inventory"),

        // Vendors/Marketplace
        ("manage_rc_vendors", "Manage Vendors"),

        // Auctions
        ("manage_rc_auctions", "Manage Auctions"),

        // Invoices
        ("manage_rc_invoices", "Manage Invoices"),

        // Advanced
        ("export_rc_data", "Export Data"),
        ("import_rc_data", "Import Data"),
    ]
}

/// Get all admin routes for the plugin
pub fn get_admin_routes() -> Vec<AdminRoute> {
    vec![
        // Dashboard
        AdminRoute::new("GET", "/admin/plugins/rustcommerce", "dashboard::index"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/home", "dashboard::home"),

        // Products
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/products", "products::list"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/products/new", "products::create"),
        AdminRoute::new("POST", "/admin/plugins/rustcommerce/products", "products::store"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/products/:id", "products::edit"),
        AdminRoute::new("PUT", "/admin/plugins/rustcommerce/products/:id", "products::update"),
        AdminRoute::new("DELETE", "/admin/plugins/rustcommerce/products/:id", "products::delete"),

        // Orders
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/orders", "orders::list"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/orders/:id", "orders::show"),
        AdminRoute::new("PUT", "/admin/plugins/rustcommerce/orders/:id", "orders::update"),

        // Customers
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/customers", "customers::list"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/customers/:id", "customers::show"),

        // Settings
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/settings", "settings::index"),
        AdminRoute::new("POST", "/admin/plugins/rustcommerce/settings", "settings::update"),

        // Analytics
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/analytics", "analytics::index"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/analytics/sales", "analytics::sales"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/analytics/products", "analytics::products"),
        AdminRoute::new("GET", "/admin/plugins/rustcommerce/analytics/customers", "analytics::customers"),
    ]
}

/// Admin route definition
#[derive(Debug, Clone)]
pub struct AdminRoute {
    pub method: String,
    pub path: String,
    pub handler: String,
}

impl AdminRoute {
    pub fn new(method: &str, path: &str, handler: &str) -> Self {
        Self {
            method: method.to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        }
    }
}
