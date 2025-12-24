//! RustCommerce Business Logic Services
//!
//! Service layer that implements the core e-commerce functionality.

pub mod pricing;
pub mod inventory;
pub mod cart;
pub mod checkout;
pub mod order;
pub mod customer;
pub mod shipping;
pub mod tax;
pub mod coupon;
pub mod product;
pub mod report;

pub use pricing::PricingService;
pub use inventory::InventoryService;
pub use cart::CartService;
pub use checkout::CheckoutService;
pub use order::OrderService;
pub use customer::CustomerService;
pub use shipping::ShippingService;
pub use tax::TaxService;
pub use coupon::CouponService;
pub use product::ProductService;
pub use report::ReportService;
