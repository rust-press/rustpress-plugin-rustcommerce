//! Checkout Service
//!
//! Handles the checkout process, order creation, and payment processing.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::cart::Cart;
use crate::models::order::{Order, OrderItem, OrderStatus, OrderAddress};
use crate::models::payment::{PaymentRequest, PaymentResult};
use crate::models::customer::Customer;
use crate::settings::RustCommerceSettings;

/// Checkout service
pub struct CheckoutService {
    settings: RustCommerceSettings,
}

/// Checkout validation result
#[derive(Debug)]
pub struct CheckoutValidation {
    pub is_valid: bool,
    pub errors: Vec<CheckoutError>,
    pub warnings: Vec<String>,
}

/// Checkout errors
#[derive(Debug, Clone)]
pub enum CheckoutError {
    CartEmpty,
    InvalidBillingAddress(String),
    InvalidShippingAddress(String),
    NoShippingMethod,
    NoPaymentMethod,
    StockError { product_id: Uuid, message: String },
    CouponError(String),
    PaymentError(String),
    CustomerRequired,
    TermsNotAccepted,
    InvalidEmail,
}

impl std::fmt::Display for CheckoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CartEmpty => write!(f, "Your cart is empty"),
            Self::InvalidBillingAddress(msg) => write!(f, "Invalid billing address: {}", msg),
            Self::InvalidShippingAddress(msg) => write!(f, "Invalid shipping address: {}", msg),
            Self::NoShippingMethod => write!(f, "Please select a shipping method"),
            Self::NoPaymentMethod => write!(f, "Please select a payment method"),
            Self::StockError { message, .. } => write!(f, "Stock error: {}", message),
            Self::CouponError(msg) => write!(f, "Coupon error: {}", msg),
            Self::PaymentError(msg) => write!(f, "Payment error: {}", msg),
            Self::CustomerRequired => write!(f, "Customer information required"),
            Self::TermsNotAccepted => write!(f, "Please accept the terms and conditions"),
            Self::InvalidEmail => write!(f, "Please enter a valid email address"),
        }
    }
}

/// Checkout request
#[derive(Debug, Clone)]
pub struct CheckoutRequest {
    pub cart_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub billing_email: String,
    pub billing_first_name: String,
    pub billing_last_name: String,
    pub billing_address: OrderAddress,
    pub shipping_address: Option<OrderAddress>,
    pub ship_to_different_address: bool,
    pub payment_method: String,
    pub payment_token: Option<Uuid>,
    pub customer_note: Option<String>,
    pub create_account: bool,
    pub accept_terms: bool,
}

/// Checkout result
#[derive(Debug)]
pub struct CheckoutResult {
    pub order_id: Uuid,
    pub order_number: String,
    pub redirect_url: Option<String>,
    pub requires_payment_action: bool,
    pub payment_action_url: Option<String>,
}

impl CheckoutService {
    /// Create a new checkout service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Validate checkout data
    pub fn validate(&self, cart: &Cart, request: &CheckoutRequest) -> CheckoutValidation {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check cart is not empty
        if cart.items.is_empty() {
            errors.push(CheckoutError::CartEmpty);
        }

        // Validate email
        if !self.is_valid_email(&request.billing_email) {
            errors.push(CheckoutError::InvalidEmail);
        }

        // Validate billing address
        if let Err(msg) = self.validate_address(&request.billing_address) {
            errors.push(CheckoutError::InvalidBillingAddress(msg));
        }

        // Validate shipping address if needed
        if request.ship_to_different_address {
            if let Some(ref shipping) = request.shipping_address {
                if let Err(msg) = self.validate_address(shipping) {
                    errors.push(CheckoutError::InvalidShippingAddress(msg));
                }
            } else {
                errors.push(CheckoutError::InvalidShippingAddress(
                    "Shipping address required".to_string()
                ));
            }
        }

        // Check shipping method (if cart needs shipping)
        if self.cart_needs_shipping(cart) && cart.shipping_method_id.is_none() {
            errors.push(CheckoutError::NoShippingMethod);
        }

        // Check payment method
        if request.payment_method.is_empty() {
            errors.push(CheckoutError::NoPaymentMethod);
        }

        // Check terms acceptance (if required)
        if self.settings.checkout.terms_page_id.is_some() && !request.accept_terms {
            errors.push(CheckoutError::TermsNotAccepted);
        }

        CheckoutValidation {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Create order from cart
    pub fn create_order(&self, cart: &Cart, request: &CheckoutRequest) -> Order {
        let order_id = Uuid::now_v7();
        let order_number = self.generate_order_number();

        // Determine shipping address
        let (shipping_first_name, shipping_last_name, shipping_address) =
            if request.ship_to_different_address {
                if let Some(ref addr) = request.shipping_address {
                    (
                        addr.first_name.clone().unwrap_or_default(),
                        addr.last_name.clone().unwrap_or_default(),
                        addr.clone(),
                    )
                } else {
                    (
                        request.billing_first_name.clone(),
                        request.billing_last_name.clone(),
                        request.billing_address.clone(),
                    )
                }
            } else {
                (
                    request.billing_first_name.clone(),
                    request.billing_last_name.clone(),
                    request.billing_address.clone(),
                )
            };

        // Create order items
        let items: Vec<OrderItem> = cart.items.iter().map(|item| {
            OrderItem {
                id: Uuid::now_v7(),
                order_id,
                product_id: item.product_id,
                variation_id: item.variation_id,
                product_name: item.product_name.clone(),
                variation_attributes: item.variation_attributes.clone(),
                quantity: item.quantity,
                unit_price: item.unit_price,
                subtotal: item.line_subtotal,
                subtotal_tax: item.line_subtotal_tax,
                total: item.line_total,
                total_tax: item.line_tax,
                sku: None,
                meta: item.meta.clone(),
            }
        }).collect();

        Order {
            id: order_id,
            site_id: cart.site_id,
            order_number,
            customer_id: request.customer_id,
            status: OrderStatus::Pending,
            currency: self.settings.general.currency.clone(),

            // Pricing
            subtotal: cart.totals.subtotal,
            discount_total: cart.totals.discount_total,
            discount_tax: cart.totals.discount_tax,
            shipping_total: cart.totals.shipping_total,
            shipping_tax: cart.totals.shipping_tax,
            cart_tax: cart.totals.subtotal_tax,
            total_tax: cart.totals.tax_total,
            total: cart.totals.total,

            // Payment
            payment_method: request.payment_method.clone(),
            payment_method_title: request.payment_method.clone(), // Would be looked up
            transaction_id: None,

            // Billing
            billing_email: request.billing_email.clone(),
            billing_first_name: request.billing_first_name.clone(),
            billing_last_name: request.billing_last_name.clone(),
            billing_address: request.billing_address.clone(),

            // Shipping
            shipping_first_name,
            shipping_last_name,
            shipping_address,

            // Items
            items,
            shipping_lines: Vec::new(),
            tax_lines: Vec::new(),
            fee_lines: Vec::new(),
            coupon_lines: cart.coupons.iter().map(|c| {
                crate::models::order::OrderCouponLine {
                    id: Uuid::now_v7(),
                    order_id,
                    code: c.code.clone(),
                    discount: c.discount,
                    discount_tax: Decimal::ZERO,
                }
            }).collect(),

            // Meta
            customer_ip: None,
            customer_user_agent: None,
            customer_note: request.customer_note.clone(),

            // Dates
            created_at: chrono::Utc::now(),
            updated_at: None,
            paid_at: None,
            completed_at: None,
        }
    }

    /// Generate unique order number
    fn generate_order_number(&self) -> String {
        // Format: RC-YYYYMMDD-XXXX
        let now = chrono::Utc::now();
        let date_part = now.format("%Y%m%d").to_string();
        let random_part: u16 = rand::random::<u16>() % 10000;
        format!("RC-{}-{:04}", date_part, random_part)
    }

    /// Validate email format
    fn is_valid_email(&self, email: &str) -> bool {
        // Basic email validation
        email.contains('@') && email.contains('.') && email.len() >= 5
    }

    /// Validate address
    fn validate_address(&self, address: &OrderAddress) -> Result<(), String> {
        if address.address_1.trim().is_empty() {
            return Err("Street address is required".to_string());
        }
        if address.city.trim().is_empty() {
            return Err("City is required".to_string());
        }
        if address.country.trim().is_empty() {
            return Err("Country is required".to_string());
        }
        if address.postcode.trim().is_empty() {
            return Err("Postal code is required".to_string());
        }
        Ok(())
    }

    /// Check if cart needs shipping
    fn cart_needs_shipping(&self, cart: &Cart) -> bool {
        // In full implementation, check if items are virtual
        !cart.items.is_empty()
    }

    /// Process payment for order
    pub async fn process_payment(
        &self,
        order: &mut Order,
        payment_request: PaymentRequest,
    ) -> Result<PaymentResult, CheckoutError> {
        // This would integrate with actual payment gateways
        // For now, return a mock success
        let result = PaymentResult::success(format!("txn_{}", Uuid::now_v7()));

        if result.success {
            order.transaction_id = result.transaction_id.clone();
            order.status = OrderStatus::Processing;
            order.paid_at = Some(chrono::Utc::now());
        }

        Ok(result)
    }

    /// Get checkout fields for a country
    pub fn get_checkout_fields(&self, country: &str) -> CheckoutFields {
        // Return default fields - in full implementation, customize by country
        CheckoutFields {
            billing: vec![
                CheckoutField::new("first_name", "First name", true),
                CheckoutField::new("last_name", "Last name", true),
                CheckoutField::new("company", "Company", false),
                CheckoutField::new("address_1", "Street address", true),
                CheckoutField::new("address_2", "Apartment, suite, etc.", false),
                CheckoutField::new("city", "City", true),
                CheckoutField::new("state", "State / Province", true),
                CheckoutField::new("postcode", "Postal code", true),
                CheckoutField::new("country", "Country", true),
                CheckoutField::new("phone", "Phone", false),
                CheckoutField::new("email", "Email", true),
            ],
            shipping: vec![
                CheckoutField::new("first_name", "First name", true),
                CheckoutField::new("last_name", "Last name", true),
                CheckoutField::new("company", "Company", false),
                CheckoutField::new("address_1", "Street address", true),
                CheckoutField::new("address_2", "Apartment, suite, etc.", false),
                CheckoutField::new("city", "City", true),
                CheckoutField::new("state", "State / Province", true),
                CheckoutField::new("postcode", "Postal code", true),
                CheckoutField::new("country", "Country", true),
            ],
            order: vec![
                CheckoutField::new("customer_note", "Order notes", false),
            ],
        }
    }

    /// Calculate order totals
    pub fn calculate_order_totals(&self, order: &mut Order) {
        let mut subtotal = Decimal::ZERO;
        let mut total_tax = Decimal::ZERO;

        for item in &order.items {
            subtotal += item.subtotal;
            total_tax += item.subtotal_tax + item.total_tax;
        }

        let shipping_total: Decimal = order.shipping_lines.iter()
            .map(|s| s.total)
            .sum();
        let shipping_tax: Decimal = order.shipping_lines.iter()
            .map(|s| s.total_tax)
            .sum();

        let fee_total: Decimal = order.fee_lines.iter()
            .map(|f| f.total)
            .sum();
        let fee_tax: Decimal = order.fee_lines.iter()
            .map(|f| f.total_tax)
            .sum();

        let discount_total: Decimal = order.coupon_lines.iter()
            .map(|c| c.discount)
            .sum();

        order.subtotal = subtotal;
        order.shipping_total = shipping_total;
        order.shipping_tax = shipping_tax;
        order.cart_tax = total_tax;
        order.total_tax = total_tax + shipping_tax + fee_tax;
        order.discount_total = discount_total;
        order.total = subtotal + shipping_total + fee_total + order.total_tax - discount_total;
    }
}

/// Checkout fields configuration
#[derive(Debug, Clone)]
pub struct CheckoutFields {
    pub billing: Vec<CheckoutField>,
    pub shipping: Vec<CheckoutField>,
    pub order: Vec<CheckoutField>,
}

/// Single checkout field
#[derive(Debug, Clone)]
pub struct CheckoutField {
    pub name: String,
    pub label: String,
    pub required: bool,
    pub field_type: String,
    pub placeholder: Option<String>,
    pub options: Vec<(String, String)>,
}

impl CheckoutField {
    pub fn new(name: &str, label: &str, required: bool) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            required,
            field_type: "text".to_string(),
            placeholder: None,
            options: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        let settings = RustCommerceSettings::default();
        let service = CheckoutService::new(settings);

        assert!(service.is_valid_email("test@example.com"));
        assert!(service.is_valid_email("user.name@domain.co.uk"));
        assert!(!service.is_valid_email("invalid"));
        assert!(!service.is_valid_email("@."));
    }

    #[test]
    fn test_order_number_generation() {
        let settings = RustCommerceSettings::default();
        let service = CheckoutService::new(settings);

        let order_number = service.generate_order_number();
        assert!(order_number.starts_with("RC-"));
        assert!(order_number.len() > 10);
    }
}
