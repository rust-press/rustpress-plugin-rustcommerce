//! Cart Service
//!
//! Handles shopping cart operations, calculations, and validation.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::cart::{Cart, CartItem, CartTotals, AppliedCoupon, CartFee};
use crate::models::product::{Product, ProductVariation};
use crate::models::coupon::Coupon;
use crate::services::pricing::PricingService;
use crate::services::inventory::InventoryService;
use crate::settings::RustCommerceSettings;

/// Cart service
pub struct CartService {
    settings: RustCommerceSettings,
    pricing_service: PricingService,
}

/// Cart operation result
#[derive(Debug)]
pub enum CartResult<T> {
    Success(T),
    Error(CartError),
}

/// Cart errors
#[derive(Debug, Clone)]
pub enum CartError {
    ProductNotFound,
    VariationNotFound,
    InsufficientStock { available: i32, requested: i32 },
    InvalidQuantity,
    ItemNotInCart,
    CouponNotValid(String),
    CartEmpty,
    MaxQuantityExceeded { max: i32 },
    ProductNotPurchasable,
}

impl std::fmt::Display for CartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProductNotFound => write!(f, "Product not found"),
            Self::VariationNotFound => write!(f, "Product variation not found"),
            Self::InsufficientStock { available, requested } => {
                write!(f, "Only {} items available, {} requested", available, requested)
            }
            Self::InvalidQuantity => write!(f, "Invalid quantity"),
            Self::ItemNotInCart => write!(f, "Item not in cart"),
            Self::CouponNotValid(msg) => write!(f, "Coupon not valid: {}", msg),
            Self::CartEmpty => write!(f, "Cart is empty"),
            Self::MaxQuantityExceeded { max } => write!(f, "Maximum quantity of {} exceeded", max),
            Self::ProductNotPurchasable => write!(f, "Product cannot be purchased"),
        }
    }
}

impl CartService {
    /// Create a new cart service
    pub fn new(settings: RustCommerceSettings) -> Self {
        let pricing_service = PricingService::new(settings.clone());
        Self {
            settings,
            pricing_service,
        }
    }

    /// Create a new empty cart
    pub fn create_cart(&self, customer_id: Option<Uuid>) -> Cart {
        Cart {
            id: Uuid::now_v7(),
            site_id: None,
            customer_id,
            session_key: if customer_id.is_none() {
                Some(Uuid::now_v7().to_string())
            } else {
                None
            },
            items: Vec::new(),
            coupons: Vec::new(),
            fees: Vec::new(),
            totals: CartTotals::default(),
            shipping_method_id: None,
            shipping_address: None,
            billing_address: None,
            customer_note: None,
            created_at: chrono::Utc::now(),
            updated_at: None,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        }
    }

    /// Add item to cart
    pub fn add_item(
        &self,
        cart: &mut Cart,
        product: &Product,
        variation: Option<&ProductVariation>,
        quantity: i32,
    ) -> Result<(), CartError> {
        if quantity <= 0 {
            return Err(CartError::InvalidQuantity);
        }

        // Check if product is purchasable
        if !product.is_purchasable() {
            return Err(CartError::ProductNotPurchasable);
        }

        // Generate cart item key
        let key = self.generate_item_key(product.id, variation.map(|v| v.id));

        // Check if item already exists
        if let Some(existing) = cart.items.iter_mut().find(|i| i.key == key) {
            let new_qty = existing.quantity + quantity;

            // Check sold individually
            if product.sold_individually && new_qty > 1 {
                return Err(CartError::MaxQuantityExceeded { max: 1 });
            }

            existing.quantity = new_qty;
        } else {
            // Check sold individually
            if product.sold_individually && quantity > 1 {
                return Err(CartError::MaxQuantityExceeded { max: 1 });
            }

            // Get price
            let price = if let Some(var) = variation {
                self.pricing_service.get_variation_price(product, var)
            } else {
                self.pricing_service.get_price(product)
            }.unwrap_or(Decimal::ZERO);

            let item = CartItem {
                key: key.clone(),
                product_id: product.id,
                variation_id: variation.map(|v| v.id),
                quantity,
                unit_price: price,
                line_subtotal: price * Decimal::from(quantity),
                line_total: price * Decimal::from(quantity),
                line_tax: Decimal::ZERO,
                line_subtotal_tax: Decimal::ZERO,
                product_name: product.name.clone(),
                variation_attributes: variation.map(|v| v.attributes.clone()).unwrap_or_default(),
                meta: HashMap::new(),
            };

            cart.items.push(item);
        }

        // Update cart hash
        cart.update_hash();
        cart.updated_at = Some(chrono::Utc::now());

        Ok(())
    }

    /// Remove item from cart
    pub fn remove_item(&self, cart: &mut Cart, key: &str) -> Result<CartItem, CartError> {
        let index = cart.items.iter().position(|i| i.key == key)
            .ok_or(CartError::ItemNotInCart)?;

        let item = cart.items.remove(index);
        cart.update_hash();
        cart.updated_at = Some(chrono::Utc::now());

        Ok(item)
    }

    /// Update item quantity
    pub fn update_quantity(
        &self,
        cart: &mut Cart,
        key: &str,
        quantity: i32,
    ) -> Result<(), CartError> {
        if quantity <= 0 {
            return self.remove_item(cart, key).map(|_| ());
        }

        let item = cart.items.iter_mut()
            .find(|i| i.key == key)
            .ok_or(CartError::ItemNotInCart)?;

        item.quantity = quantity;
        item.line_subtotal = item.unit_price * Decimal::from(quantity);
        item.line_total = item.line_subtotal;

        cart.update_hash();
        cart.updated_at = Some(chrono::Utc::now());

        Ok(())
    }

    /// Apply coupon to cart
    pub fn apply_coupon(
        &self,
        cart: &mut Cart,
        coupon: &Coupon,
    ) -> Result<(), CartError> {
        // Check if coupon already applied
        if cart.coupons.iter().any(|c| c.code == coupon.code) {
            return Err(CartError::CouponNotValid("Coupon already applied".to_string()));
        }

        // Check individual use
        if coupon.individual_use && !cart.coupons.is_empty() {
            return Err(CartError::CouponNotValid(
                "This coupon cannot be used with other coupons".to_string()
            ));
        }

        // Check if any existing coupon is individual use
        if cart.coupons.iter().any(|c| c.individual_use) {
            return Err(CartError::CouponNotValid(
                "Cannot add another coupon when an individual use coupon is applied".to_string()
            ));
        }

        // Calculate discount
        let discount = self.calculate_coupon_discount(cart, coupon);

        let applied = AppliedCoupon {
            code: coupon.code.clone(),
            coupon_id: coupon.id,
            discount_type: coupon.discount_type,
            amount: coupon.amount,
            discount: discount,
            free_shipping: coupon.free_shipping,
            individual_use: coupon.individual_use,
        };

        cart.coupons.push(applied);
        cart.update_hash();
        cart.updated_at = Some(chrono::Utc::now());

        Ok(())
    }

    /// Remove coupon from cart
    pub fn remove_coupon(&self, cart: &mut Cart, code: &str) -> Result<(), CartError> {
        let index = cart.coupons.iter().position(|c| c.code == code)
            .ok_or(CartError::CouponNotValid("Coupon not in cart".to_string()))?;

        cart.coupons.remove(index);
        cart.update_hash();
        cart.updated_at = Some(chrono::Utc::now());

        Ok(())
    }

    /// Add fee to cart
    pub fn add_fee(&self, cart: &mut Cart, fee: CartFee) {
        cart.fees.push(fee);
        cart.update_hash();
        cart.updated_at = Some(chrono::Utc::now());
    }

    /// Calculate cart totals
    pub fn calculate_totals(&self, cart: &mut Cart) {
        let mut subtotal = Decimal::ZERO;
        let mut subtotal_tax = Decimal::ZERO;

        // Calculate line totals
        for item in &mut cart.items {
            item.line_subtotal = item.unit_price * Decimal::from(item.quantity);
            item.line_total = item.line_subtotal;
            subtotal += item.line_subtotal;
            subtotal_tax += item.line_subtotal_tax;
        }

        // Calculate discount
        let mut discount_total = Decimal::ZERO;
        for coupon in &cart.coupons {
            discount_total += coupon.discount;
        }

        // Calculate shipping
        let shipping_total = cart.totals.shipping_total;
        let shipping_tax = cart.totals.shipping_tax;

        // Calculate fees
        let mut fee_total = Decimal::ZERO;
        let mut fee_tax = Decimal::ZERO;
        for fee in &cart.fees {
            fee_total += fee.amount;
            fee_tax += fee.tax;
        }

        // Calculate tax
        let tax_total = subtotal_tax + shipping_tax + fee_tax;

        // Calculate total
        let total = subtotal + shipping_total + fee_total + tax_total - discount_total;

        cart.totals = CartTotals {
            subtotal,
            subtotal_tax,
            shipping_total,
            shipping_tax,
            discount_total,
            discount_tax: Decimal::ZERO,
            fee_total,
            fee_tax,
            tax_total,
            total: total.max(Decimal::ZERO),
        };
    }

    /// Calculate discount from a coupon
    fn calculate_coupon_discount(&self, cart: &Cart, coupon: &Coupon) -> Decimal {
        use crate::models::coupon::DiscountType;

        match coupon.discount_type {
            DiscountType::Percent => {
                let subtotal: Decimal = cart.items.iter()
                    .filter(|item| coupon.applies_to_product(item.product_id))
                    .map(|item| item.line_subtotal)
                    .sum();

                let discount = subtotal * (coupon.amount / dec!(100));

                // Apply maximum discount if set
                if let Some(max) = coupon.maximum_amount {
                    discount.min(max)
                } else {
                    discount
                }
            }
            DiscountType::FixedCart => {
                coupon.amount.min(cart.totals.subtotal)
            }
            DiscountType::FixedProduct => {
                cart.items.iter()
                    .filter(|item| coupon.applies_to_product(item.product_id))
                    .map(|item| coupon.amount * Decimal::from(item.quantity))
                    .sum::<Decimal>()
                    .min(cart.totals.subtotal)
            }
        }
    }

    /// Generate unique key for cart item
    fn generate_item_key(&self, product_id: Uuid, variation_id: Option<Uuid>) -> String {
        match variation_id {
            Some(vid) => format!("{}_{}", product_id, vid),
            None => product_id.to_string(),
        }
    }

    /// Clear all items from cart
    pub fn clear(&self, cart: &mut Cart) {
        cart.items.clear();
        cart.coupons.clear();
        cart.fees.clear();
        cart.totals = CartTotals::default();
        cart.update_hash();
        cart.updated_at = Some(chrono::Utc::now());
    }

    /// Get cart item count
    pub fn get_item_count(&self, cart: &Cart) -> i32 {
        cart.items.iter().map(|i| i.quantity).sum()
    }

    /// Check if cart is empty
    pub fn is_empty(&self, cart: &Cart) -> bool {
        cart.items.is_empty()
    }

    /// Check if cart needs shipping
    pub fn needs_shipping(&self, cart: &Cart) -> bool {
        // For now, assume all non-virtual products need shipping
        // In full implementation, would check each product's virtual flag
        !cart.items.is_empty()
    }

    /// Get cart subtotal
    pub fn get_subtotal(&self, cart: &Cart) -> Decimal {
        cart.items.iter().map(|i| i.line_subtotal).sum()
    }

    /// Check if free shipping is available (via coupon)
    pub fn has_free_shipping(&self, cart: &Cart) -> bool {
        cart.coupons.iter().any(|c| c.free_shipping)
    }

    /// Validate cart items (check stock, prices, etc.)
    pub fn validate_cart(
        &self,
        cart: &Cart,
        inventory_service: &InventoryService,
        products: &HashMap<Uuid, Product>,
        variations: &HashMap<Uuid, ProductVariation>,
    ) -> Vec<CartError> {
        let mut errors = Vec::new();

        for item in &cart.items {
            // Get product
            let product = match products.get(&item.product_id) {
                Some(p) => p,
                None => {
                    errors.push(CartError::ProductNotFound);
                    continue;
                }
            };

            // Check variation if needed
            if let Some(var_id) = item.variation_id {
                let variation = match variations.get(&var_id) {
                    Some(v) => v,
                    None => {
                        errors.push(CartError::VariationNotFound);
                        continue;
                    }
                };

                // Check stock
                let stock_result = inventory_service.check_variation_stock(
                    product,
                    variation,
                    item.quantity,
                );

                if !stock_result.is_available {
                    errors.push(CartError::InsufficientStock {
                        available: stock_result.available_quantity.unwrap_or(0),
                        requested: item.quantity,
                    });
                }
            } else {
                // Check product stock
                let stock_result = inventory_service.check_stock(product, item.quantity);

                if !stock_result.is_available {
                    errors.push(CartError::InsufficientStock {
                        available: stock_result.available_quantity.unwrap_or(0),
                        requested: item.quantity,
                    });
                }
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_item_key() {
        let settings = RustCommerceSettings::default();
        let service = CartService::new(settings);

        let product_id = Uuid::new_v4();
        let variation_id = Uuid::new_v4();

        let key1 = service.generate_item_key(product_id, None);
        assert_eq!(key1, product_id.to_string());

        let key2 = service.generate_item_key(product_id, Some(variation_id));
        assert_eq!(key2, format!("{}_{}", product_id, variation_id));
    }

    #[test]
    fn test_cart_item_count() {
        let settings = RustCommerceSettings::default();
        let service = CartService::new(settings);

        let mut cart = service.create_cart(None);
        assert_eq!(service.get_item_count(&cart), 0);
        assert!(service.is_empty(&cart));
    }
}
