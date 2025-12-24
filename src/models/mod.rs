//! RustCommerce Data Models
//!
//! Comprehensive e-commerce data models including:
//! - Core: Products, Orders, Customers, Cart, Payments
//! - Features: Subscriptions, Bookings, Memberships, Rewards
//! - Marketing: Email Marketing, Cart Recovery, Recommendations
//! - Advanced: Auctions, Marketplace, Dynamic Pricing, Analytics

// Core modules
pub mod product;
pub mod order;
pub mod customer;
pub mod cart;
pub mod coupon;
pub mod shipping;
pub mod tax;
pub mod payment;

// Enhanced features
pub mod subscription;
pub mod booking;
pub mod membership;
pub mod rewards;
pub mod wishlist;
pub mod bundle;
pub mod auction;
pub mod vendor;

// Analytics and Marketing
pub mod analytics;
pub mod email_marketing;
pub mod cart_recovery;
pub mod recommendations;

// Advanced features
pub mod dynamic_pricing;
pub mod inventory;
pub mod currency;
pub mod customization;
pub mod preorder;
pub mod waitlist;
pub mod comparison;
pub mod search;

// Management
pub mod invoice;
pub mod email_templates;

// Re-export core types
pub use product::*;
pub use order::*;
pub use customer::*;
pub use cart::*;
pub use coupon::*;
pub use shipping::*;
pub use tax::*;
pub use payment::*;

// Re-export enhanced features
pub use subscription::*;
pub use booking::*;
pub use membership::*;
pub use rewards::*;
pub use wishlist::*;
pub use bundle::*;
pub use auction::*;
pub use vendor::*;

// Re-export analytics and marketing
pub use analytics::*;
pub use email_marketing::*;
pub use cart_recovery::*;
pub use recommendations::*;

// Re-export advanced features
pub use dynamic_pricing::*;
pub use inventory::*;
pub use currency::*;
pub use customization::*;
pub use preorder::*;
pub use waitlist::*;
pub use comparison::*;
pub use search::*;

// Re-export management
pub use invoice::*;
pub use email_templates::*;
