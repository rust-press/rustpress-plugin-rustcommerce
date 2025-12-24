//! Cart Models

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use super::customer::Address;
use super::product::{Product, ProductVariation};

/// Shopping cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub session_key: Option<String>,
    pub customer_id: Option<Uuid>,

    // Items
    pub items: Vec<CartItem>,

    // Coupons
    pub applied_coupons: Vec<AppliedCoupon>,

    // Addresses (for shipping calculation)
    pub billing_address: Option<Address>,
    pub shipping_address: Option<Address>,

    // Selected shipping method
    pub chosen_shipping_method: Option<String>,

    // Totals (calculated)
    pub totals: CartTotals,

    // Fees
    pub fees: Vec<CartFee>,

    // Metadata
    pub meta: HashMap<String, serde_json::Value>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Cart {
    /// Create a new empty cart
    pub fn new(session_key: Option<String>, customer_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            site_id: None,
            session_key,
            customer_id,
            items: Vec::new(),
            applied_coupons: Vec::new(),
            billing_address: None,
            shipping_address: None,
            chosen_shipping_method: None,
            totals: CartTotals::default(),
            fees: Vec::new(),
            meta: HashMap::new(),
            created_at: now,
            updated_at: now,
            expires_at: now + chrono::Duration::days(30),
        }
    }

    /// Check if cart is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get total item count
    pub fn get_item_count(&self) -> i32 {
        self.items.iter().map(|i| i.quantity).sum()
    }

    /// Get unique product count
    pub fn get_product_count(&self) -> usize {
        self.items.len()
    }

    /// Find item by product and variation
    pub fn find_item(&self, product_id: Uuid, variation_id: Option<Uuid>) -> Option<&CartItem> {
        self.items.iter().find(|i| {
            i.product_id == product_id && i.variation_id == variation_id
        })
    }

    /// Find item by key
    pub fn find_item_by_key(&self, key: &str) -> Option<&CartItem> {
        self.items.iter().find(|i| i.key == key)
    }

    /// Check if cart needs shipping
    pub fn needs_shipping(&self) -> bool {
        self.items.iter().any(|i| !i.is_virtual)
    }

    /// Check if cart has only virtual items
    pub fn is_virtual(&self) -> bool {
        !self.items.is_empty() && self.items.iter().all(|i| i.is_virtual)
    }

    /// Calculate cart hash (for duplicate detection)
    pub fn calculate_hash(&self) -> String {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();

        // Add items to hash
        for item in &self.items {
            hasher.update(item.product_id.to_string());
            if let Some(vid) = item.variation_id {
                hasher.update(vid.to_string());
            }
            hasher.update(item.quantity.to_string());
        }

        // Add coupons to hash
        for coupon in &self.applied_coupons {
            hasher.update(&coupon.code);
        }

        hex::encode(hasher.finalize())
    }
}

/// Cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    /// Unique key for this cart item
    pub key: String,

    /// Product ID
    pub product_id: Uuid,

    /// Variation ID (if applicable)
    pub variation_id: Option<Uuid>,

    /// Quantity
    pub quantity: i32,

    /// Product data (snapshot at time of adding)
    pub product_name: String,
    pub product_sku: Option<String>,
    pub product_image: Option<String>,

    /// Variation attributes (for display)
    pub variation_attributes: HashMap<String, String>,

    /// Pricing
    pub price: Decimal,           // Unit price
    pub regular_price: Decimal,   // Regular unit price
    pub subtotal: Decimal,        // price * quantity
    pub subtotal_tax: Decimal,
    pub total: Decimal,           // After discounts
    pub total_tax: Decimal,

    /// Flags
    pub is_virtual: bool,
    pub is_downloadable: bool,
    pub sold_individually: bool,

    /// Stock
    pub stock_quantity: Option<i32>,
    pub backorders_allowed: bool,

    /// Weight and dimensions
    pub weight: Option<Decimal>,
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,

    /// Tax
    pub tax_class: String,
    pub taxes: HashMap<Uuid, Decimal>, // tax_rate_id -> amount

    /// Metadata (for addons, custom fields, etc.)
    pub meta: HashMap<String, serde_json::Value>,

    /// Timestamps
    pub added_at: DateTime<Utc>,
}

impl CartItem {
    /// Generate unique key for cart item
    pub fn generate_key(product_id: Uuid, variation_id: Option<Uuid>, meta: &HashMap<String, serde_json::Value>) -> String {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.update(product_id.to_string());

        if let Some(vid) = variation_id {
            hasher.update(vid.to_string());
        }

        // Include meta in key so different addons create different items
        if !meta.is_empty() {
            let meta_str = serde_json::to_string(meta).unwrap_or_default();
            hasher.update(meta_str);
        }

        hex::encode(&hasher.finalize()[..16]) // Use first 16 bytes
    }

    /// Create from product
    pub fn from_product(
        product: &Product,
        quantity: i32,
        variation: Option<&ProductVariation>,
        meta: HashMap<String, serde_json::Value>,
    ) -> Self {
        let variation_id = variation.map(|v| v.id);
        let key = Self::generate_key(product.id, variation_id, &meta);

        // Get price from variation or product
        let (price, regular_price) = if let Some(v) = variation {
            (
                v.sale_price.or(v.regular_price).unwrap_or(Decimal::ZERO),
                v.regular_price.unwrap_or(Decimal::ZERO),
            )
        } else {
            (
                product.get_price().unwrap_or(Decimal::ZERO),
                product.regular_price.unwrap_or(Decimal::ZERO),
            )
        };

        let subtotal = price * Decimal::from(quantity);

        // Build variation attributes for display
        let variation_attributes = if let Some(v) = variation {
            v.attributes
                .iter()
                .map(|a| (a.attribute_name.clone(), a.term_name.clone().unwrap_or_default()))
                .collect()
        } else {
            HashMap::new()
        };

        Self {
            key,
            product_id: product.id,
            variation_id,
            quantity,
            product_name: product.name.clone(),
            product_sku: variation.and_then(|v| v.sku.clone()).or(product.sku.clone()),
            product_image: None, // Would load from product gallery
            variation_attributes,
            price,
            regular_price,
            subtotal,
            subtotal_tax: Decimal::ZERO,
            total: subtotal,
            total_tax: Decimal::ZERO,
            is_virtual: variation.and_then(|v| v.is_virtual).unwrap_or(product.is_virtual),
            is_downloadable: variation.and_then(|v| v.is_downloadable).unwrap_or(product.is_downloadable),
            sold_individually: product.sold_individually,
            stock_quantity: variation.and_then(|v| v.stock_quantity).or(Some(product.stock_quantity)),
            backorders_allowed: product.backorders != super::product::BackorderStatus::No,
            weight: variation.and_then(|v| v.weight).or(product.weight),
            length: variation.and_then(|v| v.length).or(product.length),
            width: variation.and_then(|v| v.width).or(product.width),
            height: variation.and_then(|v| v.height).or(product.height),
            tax_class: product.tax_class.clone(),
            taxes: HashMap::new(),
            meta,
            added_at: Utc::now(),
        }
    }

    /// Update quantity
    pub fn set_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
        self.subtotal = self.price * Decimal::from(quantity);
        self.total = self.subtotal; // Will be updated by cart calculations
    }

    /// Get total weight
    pub fn get_total_weight(&self) -> Decimal {
        self.weight.unwrap_or(Decimal::ZERO) * Decimal::from(self.quantity)
    }

    /// Check if item is on sale
    pub fn is_on_sale(&self) -> bool {
        self.price < self.regular_price
    }
}

/// Applied coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedCoupon {
    pub code: String,
    pub coupon_id: Option<Uuid>,
    pub discount_type: String,
    pub amount: Decimal,
    pub discount: Decimal,        // Total discount amount
    pub discount_tax: Decimal,
}

/// Cart fee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartFee {
    pub id: String,
    pub name: String,
    pub amount: Decimal,
    pub tax_class: String,
    pub taxable: bool,
    pub total: Decimal,
    pub total_tax: Decimal,
}

/// Cart totals
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartTotals {
    pub subtotal: Decimal,
    pub subtotal_tax: Decimal,
    pub discount_total: Decimal,
    pub discount_tax: Decimal,
    pub shipping_total: Decimal,
    pub shipping_tax: Decimal,
    pub fee_total: Decimal,
    pub fee_tax: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,

    // Tax breakdown by rate
    pub taxes: Vec<CartTaxLine>,

    // Shipping packages
    pub shipping_packages: Vec<ShippingPackage>,
}

/// Cart tax line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartTaxLine {
    pub rate_id: Uuid,
    pub rate_code: String,
    pub label: String,
    pub compound: bool,
    pub tax_total: Decimal,
    pub shipping_tax_total: Decimal,
}

/// Shipping package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingPackage {
    pub id: String,
    pub contents: Vec<CartItem>,
    pub contents_cost: Decimal,
    pub destination: Option<Address>,
    pub rates: Vec<ShippingRate>,
    pub chosen_rate: Option<String>,
}

/// Shipping rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRate {
    pub id: String,
    pub method_id: String,
    pub instance_id: String,
    pub label: String,
    pub cost: Decimal,
    pub taxes: HashMap<Uuid, Decimal>,
    pub meta: HashMap<String, String>,
}

// =============================================================================
// DTOs for API
// =============================================================================

/// Request to add item to cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub variation: Option<HashMap<String, String>>, // For variable products without variation_id
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Request to update cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCartItemRequest {
    pub key: String,
    pub quantity: i32,
}

/// Request to remove cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveCartItemRequest {
    pub key: String,
}

/// Request to apply coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCouponRequest {
    pub code: String,
}

/// Request to update shipping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShippingRequest {
    pub country: String,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub city: Option<String>,
    pub method_id: Option<String>,
}

/// Cart response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartResponse {
    pub items: Vec<CartItem>,
    pub coupons: Vec<AppliedCoupon>,
    pub fees: Vec<CartFee>,
    pub totals: CartTotals,
    pub item_count: i32,
    pub needs_shipping: bool,
    pub shipping_address: Option<Address>,
    pub available_shipping_methods: Vec<ShippingRate>,
    pub chosen_shipping_method: Option<String>,
}
