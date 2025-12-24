//! RustCommerce Payment Gateways
//!
//! Payment gateway integrations for processing payments.

pub mod gateway;
pub mod stripe;
pub mod paypal;
pub mod cod;
pub mod bacs;

pub use gateway::{PaymentGateway, PaymentGatewayRegistry};
