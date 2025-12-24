//! RustCommerce - Complete E-Commerce Solution for RustPress
//!
//! A comprehensive WooCommerce-like plugin providing:
//! - Product management (simple, variable, grouped, virtual, downloadable)
//! - Shopping cart and checkout
//! - Order management
//! - Customer management
//! - Payment gateways (Stripe, PayPal, etc.)
//! - Shipping methods and zones
//! - Tax calculations
//! - Coupons and discounts
//! - Reports and analytics
//!
//! # Architecture
//!
//! The plugin follows a layered architecture:
//! - **Models**: Data structures and database entities
//! - **Handlers**: HTTP request handlers for REST API
//! - **Services**: Business logic layer
//! - **Payments**: Payment gateway integrations
//! - **Admin**: Admin interface functionality

pub mod models;
pub mod handlers;
pub mod services;
pub mod payments;
pub mod admin;
mod plugin;
mod settings;
mod hooks;
mod shortcodes;
mod widgets;

use std::sync::Arc;
pub use plugin::RustCommercePlugin;
pub use settings::RustCommerceSettings;

/// Create the RustCommerce plugin instance
pub fn create_plugin() -> Arc<dyn rustpress_core::plugin::Plugin> {
    Arc::new(RustCommercePlugin::new())
}

/// Plugin version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin ID
pub const PLUGIN_ID: &str = "rustcommerce";

/// API namespace
pub const API_NAMESPACE: &str = "rc/v1";

// Re-export commonly used types
pub use models::product::{Product, ProductType, ProductStatus};
pub use models::order::{Order, OrderStatus, OrderItem};
pub use models::customer::Customer;
pub use models::cart::{Cart, CartItem};
pub use models::coupon::{Coupon, DiscountType};
pub use services::pricing::PricingService;
pub use services::cart::CartService;
pub use services::checkout::CheckoutService;
pub use services::inventory::InventoryService;
pub use services::shipping::ShippingService;
pub use services::tax::TaxService;
pub use payments::gateway::{PaymentGateway, PaymentGatewayRegistry};
pub use payments::stripe::StripeGateway;
pub use payments::paypal::PayPalGateway;
