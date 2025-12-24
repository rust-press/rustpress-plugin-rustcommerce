//! RustCommerce REST API Handlers
//!
//! HTTP request handlers for the RustCommerce API and Admin interface.

pub mod products;
pub mod orders;
pub mod customers;
pub mod cart;
pub mod checkout;
pub mod coupons;
pub mod shipping;
pub mod tax;
pub mod reports;
pub mod webhooks;
pub mod admin;

// Additional handlers for enhanced features
pub mod subscriptions;
pub mod bookings;
pub mod memberships;
pub mod vendors;
pub mod inventory;
pub mod marketing;
pub mod auctions;
pub mod invoices;
pub mod analytics;

pub use products::*;
pub use orders::*;
pub use customers::*;
pub use cart::*;
pub use admin::*;
