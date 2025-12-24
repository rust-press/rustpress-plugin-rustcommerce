//! Coupon Service
//!
//! Handles coupon validation, application, and discount calculations.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashSet;

use crate::models::coupon::{Coupon, DiscountType, CouponValidationResult};
use crate::models::cart::Cart;
use crate::models::customer::Customer;
use crate::settings::RustCommerceSettings;

/// Coupon service
pub struct CouponService {
    settings: RustCommerceSettings,
}

/// Coupon validation context
#[derive(Debug)]
pub struct ValidationContext<'a> {
    pub cart: &'a Cart,
    pub customer: Option<&'a Customer>,
    pub customer_email: Option<&'a str>,
    pub existing_coupons: &'a [String],
}

/// Coupon error
#[derive(Debug, Clone)]
pub enum CouponError {
    NotFound,
    Expired,
    NotYetValid,
    UsageLimitReached,
    CustomerUsageLimitReached,
    MinimumNotMet { minimum: Decimal, current: Decimal },
    MaximumExceeded { maximum: Decimal, current: Decimal },
    NotApplicable,
    IndividualUse,
    ExcludedProduct,
    ExcludedCategory,
    EmailRestriction,
    AlreadyApplied,
    InvalidCode,
}

impl std::fmt::Display for CouponError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Coupon not found"),
            Self::Expired => write!(f, "This coupon has expired"),
            Self::NotYetValid => write!(f, "This coupon is not yet valid"),
            Self::UsageLimitReached => write!(f, "Coupon usage limit has been reached"),
            Self::CustomerUsageLimitReached => write!(f, "You have already used this coupon the maximum number of times"),
            Self::MinimumNotMet { minimum, .. } => write!(f, "Minimum spend of {} is required", minimum),
            Self::MaximumExceeded { maximum, .. } => write!(f, "Maximum spend of {} exceeded", maximum),
            Self::NotApplicable => write!(f, "This coupon is not applicable to your cart"),
            Self::IndividualUse => write!(f, "This coupon cannot be used with other coupons"),
            Self::ExcludedProduct => write!(f, "This coupon does not apply to items in your cart"),
            Self::ExcludedCategory => write!(f, "This coupon does not apply to product categories in your cart"),
            Self::EmailRestriction => write!(f, "This coupon is restricted to specific email addresses"),
            Self::AlreadyApplied => write!(f, "This coupon has already been applied"),
            Self::InvalidCode => write!(f, "Invalid coupon code"),
        }
    }
}

impl CouponService {
    /// Create a new coupon service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Validate a coupon code
    pub fn validate(&self, coupon: &Coupon, context: &ValidationContext) -> Result<(), CouponError> {
        let now = Utc::now();

        // Check if already applied
        if context.existing_coupons.iter().any(|c| c.eq_ignore_ascii_case(&coupon.code)) {
            return Err(CouponError::AlreadyApplied);
        }

        // Check expiry
        if let Some(expiry) = coupon.expiry_date {
            if now > expiry {
                return Err(CouponError::Expired);
            }
        }

        // Check start date (if we add this field)
        // if let Some(start) = coupon.start_date {
        //     if now < start {
        //         return Err(CouponError::NotYetValid);
        //     }
        // }

        // Check usage limit
        if let Some(limit) = coupon.usage_limit {
            if coupon.usage_count >= limit {
                return Err(CouponError::UsageLimitReached);
            }
        }

        // Check per-user usage limit
        if let Some(limit) = coupon.usage_limit_per_user {
            if let Some(customer) = context.customer {
                let user_usage = self.get_customer_usage_count(coupon, customer.id);
                if user_usage >= limit {
                    return Err(CouponError::CustomerUsageLimitReached);
                }
            }
        }

        // Check minimum amount
        if let Some(minimum) = coupon.minimum_amount {
            if context.cart.totals.subtotal < minimum {
                return Err(CouponError::MinimumNotMet {
                    minimum,
                    current: context.cart.totals.subtotal,
                });
            }
        }

        // Check maximum amount
        if let Some(maximum) = coupon.maximum_amount {
            if context.cart.totals.subtotal > maximum {
                return Err(CouponError::MaximumExceeded {
                    maximum,
                    current: context.cart.totals.subtotal,
                });
            }
        }

        // Check individual use
        if coupon.individual_use && !context.existing_coupons.is_empty() {
            return Err(CouponError::IndividualUse);
        }

        // Check if any existing coupon is individual use
        // (Would need to look up existing coupons in full implementation)

        // Check email restrictions
        if !coupon.email_restrictions.is_empty() {
            let email = context.customer_email
                .or(context.customer.map(|c| c.email.as_str()));

            match email {
                Some(e) => {
                    let allowed = coupon.email_restrictions.iter()
                        .any(|r| {
                            if r.contains('*') {
                                // Wildcard match
                                let pattern = r.replace('*', "");
                                e.ends_with(&pattern)
                            } else {
                                r.eq_ignore_ascii_case(e)
                            }
                        });
                    if !allowed {
                        return Err(CouponError::EmailRestriction);
                    }
                }
                None => return Err(CouponError::EmailRestriction),
            }
        }

        // Check product restrictions
        if !self.check_product_restrictions(coupon, context.cart) {
            return Err(CouponError::NotApplicable);
        }

        Ok(())
    }

    /// Check product restrictions
    fn check_product_restrictions(&self, coupon: &Coupon, cart: &Cart) -> bool {
        let cart_product_ids: HashSet<Uuid> = cart.items.iter()
            .map(|i| i.product_id)
            .collect();

        // If specific products are set, at least one must be in cart
        if !coupon.product_ids.is_empty() {
            let has_product = coupon.product_ids.iter()
                .any(|id| cart_product_ids.contains(id));
            if !has_product {
                return false;
            }
        }

        // Check excluded products
        if !coupon.excluded_product_ids.is_empty() {
            let all_excluded = cart_product_ids.iter()
                .all(|id| coupon.excluded_product_ids.contains(id));
            if all_excluded {
                return false;
            }
        }

        true
    }

    /// Get customer's usage count for coupon
    fn get_customer_usage_count(&self, _coupon: &Coupon, _customer_id: Uuid) -> i32 {
        // In full implementation, query coupon_usage table
        0
    }

    /// Calculate discount for coupon
    pub fn calculate_discount(&self, coupon: &Coupon, cart: &Cart) -> Decimal {
        match coupon.discount_type {
            DiscountType::Percent => {
                self.calculate_percent_discount(coupon, cart)
            }
            DiscountType::FixedCart => {
                self.calculate_fixed_cart_discount(coupon, cart)
            }
            DiscountType::FixedProduct => {
                self.calculate_fixed_product_discount(coupon, cart)
            }
        }
    }

    /// Calculate percentage discount
    fn calculate_percent_discount(&self, coupon: &Coupon, cart: &Cart) -> Decimal {
        let applicable_total: Decimal = cart.items.iter()
            .filter(|item| coupon.applies_to_product(item.product_id))
            .map(|item| item.line_subtotal)
            .sum();

        let discount = applicable_total * (coupon.amount / dec!(100));

        // Apply limit if set
        if coupon.limit_usage_to_x_items.is_some() {
            // Would limit to X items in full implementation
        }

        // Apply maximum discount if set
        if let Some(max) = coupon.maximum_amount {
            discount.min(max)
        } else {
            discount
        }
    }

    /// Calculate fixed cart discount
    fn calculate_fixed_cart_discount(&self, coupon: &Coupon, cart: &Cart) -> Decimal {
        // Fixed cart discount applies to whole cart up to cart total
        coupon.amount.min(cart.totals.subtotal)
    }

    /// Calculate fixed product discount
    fn calculate_fixed_product_discount(&self, coupon: &Coupon, cart: &Cart) -> Decimal {
        let mut total_discount = Decimal::ZERO;

        for item in &cart.items {
            if coupon.applies_to_product(item.product_id) {
                let item_discount = coupon.amount * Decimal::from(item.quantity);
                total_discount += item_discount.min(item.line_subtotal);
            }
        }

        // Don't exceed cart subtotal
        total_discount.min(cart.totals.subtotal)
    }

    /// Generate a unique coupon code
    pub fn generate_code(&self, length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();

        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Create a simple percentage coupon
    pub fn create_percent_coupon(
        &self,
        code: String,
        amount: Decimal,
        description: Option<String>,
    ) -> Coupon {
        Coupon {
            id: Uuid::now_v7(),
            site_id: None,
            code,
            description,
            discount_type: DiscountType::Percent,
            amount,
            free_shipping: false,
            expiry_date: None,
            minimum_amount: None,
            maximum_amount: None,
            individual_use: false,
            exclude_sale_items: false,
            product_ids: Vec::new(),
            excluded_product_ids: Vec::new(),
            product_categories: Vec::new(),
            excluded_product_categories: Vec::new(),
            email_restrictions: Vec::new(),
            usage_limit: None,
            usage_limit_per_user: None,
            limit_usage_to_x_items: None,
            usage_count: 0,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    /// Create a fixed amount coupon
    pub fn create_fixed_coupon(
        &self,
        code: String,
        amount: Decimal,
        discount_type: DiscountType,
    ) -> Coupon {
        Coupon {
            id: Uuid::now_v7(),
            site_id: None,
            code,
            description: None,
            discount_type,
            amount,
            free_shipping: false,
            expiry_date: None,
            minimum_amount: None,
            maximum_amount: None,
            individual_use: false,
            exclude_sale_items: false,
            product_ids: Vec::new(),
            excluded_product_ids: Vec::new(),
            product_categories: Vec::new(),
            excluded_product_categories: Vec::new(),
            email_restrictions: Vec::new(),
            usage_limit: None,
            usage_limit_per_user: None,
            limit_usage_to_x_items: None,
            usage_count: 0,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    /// Format discount for display
    pub fn format_discount(&self, coupon: &Coupon) -> String {
        match coupon.discount_type {
            DiscountType::Percent => format!("{}%", coupon.amount),
            DiscountType::FixedCart | DiscountType::FixedProduct => {
                // Would use pricing service for proper formatting
                format!("${}", coupon.amount)
            }
        }
    }

    /// Check if coupon gives free shipping
    pub fn gives_free_shipping(&self, coupon: &Coupon) -> bool {
        coupon.free_shipping
    }

    /// Record coupon usage
    pub fn record_usage(&self, coupon: &mut Coupon, customer_id: Option<Uuid>, order_id: Uuid) {
        coupon.usage_count += 1;
        coupon.updated_at = Some(Utc::now());

        // In full implementation, also insert into coupon_usage table
    }
}

/// Coupon usage record
#[derive(Debug, Clone)]
pub struct CouponUsage {
    pub id: Uuid,
    pub coupon_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub order_id: Uuid,
    pub discount_amount: Decimal,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cart::{CartTotals, CartItem};

    fn create_test_cart(subtotal: Decimal) -> Cart {
        Cart {
            id: Uuid::new_v4(),
            site_id: None,
            customer_id: None,
            session_key: Some("test".to_string()),
            items: vec![],
            coupons: vec![],
            fees: vec![],
            totals: CartTotals {
                subtotal,
                ..Default::default()
            },
            shipping_method_id: None,
            shipping_address: None,
            billing_address: None,
            customer_note: None,
            created_at: Utc::now(),
            updated_at: None,
            expires_at: None,
        }
    }

    #[test]
    fn test_percent_discount() {
        let settings = RustCommerceSettings::default();
        let service = CouponService::new(settings);

        let coupon = service.create_percent_coupon(
            "TEST10".to_string(),
            dec!(10),
            None,
        );

        let cart = create_test_cart(dec!(100));
        let discount = service.calculate_discount(&coupon, &cart);

        assert_eq!(discount, dec!(10));
    }

    #[test]
    fn test_fixed_cart_discount() {
        let settings = RustCommerceSettings::default();
        let service = CouponService::new(settings);

        let coupon = service.create_fixed_coupon(
            "SAVE20".to_string(),
            dec!(20),
            DiscountType::FixedCart,
        );

        let cart = create_test_cart(dec!(100));
        let discount = service.calculate_discount(&coupon, &cart);

        assert_eq!(discount, dec!(20));
    }

    #[test]
    fn test_discount_cannot_exceed_cart() {
        let settings = RustCommerceSettings::default();
        let service = CouponService::new(settings);

        let coupon = service.create_fixed_coupon(
            "BIGDISCOUNT".to_string(),
            dec!(100),
            DiscountType::FixedCart,
        );

        let cart = create_test_cart(dec!(50));
        let discount = service.calculate_discount(&coupon, &cart);

        assert_eq!(discount, dec!(50)); // Capped at cart subtotal
    }

    #[test]
    fn test_generate_code() {
        let settings = RustCommerceSettings::default();
        let service = CouponService::new(settings);

        let code = service.generate_code(8);
        assert_eq!(code.len(), 8);
        assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
