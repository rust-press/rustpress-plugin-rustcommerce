//! RustCommerce Hooks
//!
//! Actions and filters for extending RustCommerce functionality.

use std::collections::HashMap;

/// Hook types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    Action,
    Filter,
}

/// Hook definition
#[derive(Debug, Clone)]
pub struct Hook {
    pub name: String,
    pub hook_type: HookType,
    pub description: String,
    pub parameters: Vec<String>,
}

/// Get all RustCommerce hooks
pub fn get_hooks() -> Vec<Hook> {
    vec![
        // Product hooks
        Hook {
            name: "rustcommerce_before_product_object_save".to_string(),
            hook_type: HookType::Action,
            description: "Fires before a product is saved".to_string(),
            parameters: vec!["product".to_string()],
        },
        Hook {
            name: "rustcommerce_product_object_updated".to_string(),
            hook_type: HookType::Action,
            description: "Fires after a product is updated".to_string(),
            parameters: vec!["product_id".to_string(), "product".to_string()],
        },
        Hook {
            name: "rustcommerce_before_delete_product".to_string(),
            hook_type: HookType::Action,
            description: "Fires before a product is deleted".to_string(),
            parameters: vec!["product_id".to_string()],
        },
        Hook {
            name: "rustcommerce_product_get_price".to_string(),
            hook_type: HookType::Filter,
            description: "Filter the product price".to_string(),
            parameters: vec!["price".to_string(), "product".to_string()],
        },

        // Cart hooks
        Hook {
            name: "rustcommerce_before_cart_item_add".to_string(),
            hook_type: HookType::Action,
            description: "Fires before an item is added to cart".to_string(),
            parameters: vec!["product_id".to_string(), "quantity".to_string()],
        },
        Hook {
            name: "rustcommerce_cart_item_added".to_string(),
            hook_type: HookType::Action,
            description: "Fires after an item is added to cart".to_string(),
            parameters: vec!["cart_item_key".to_string(), "product_id".to_string()],
        },
        Hook {
            name: "rustcommerce_cart_item_removed".to_string(),
            hook_type: HookType::Action,
            description: "Fires after an item is removed from cart".to_string(),
            parameters: vec!["cart_item_key".to_string(), "cart".to_string()],
        },
        Hook {
            name: "rustcommerce_cart_calculate_totals".to_string(),
            hook_type: HookType::Action,
            description: "Fires when cart totals are calculated".to_string(),
            parameters: vec!["cart".to_string()],
        },
        Hook {
            name: "rustcommerce_cart_subtotal".to_string(),
            hook_type: HookType::Filter,
            description: "Filter the cart subtotal".to_string(),
            parameters: vec!["subtotal".to_string(), "cart".to_string()],
        },

        // Checkout hooks
        Hook {
            name: "rustcommerce_before_checkout_process".to_string(),
            hook_type: HookType::Action,
            description: "Fires before checkout processing begins".to_string(),
            parameters: vec![],
        },
        Hook {
            name: "rustcommerce_checkout_process".to_string(),
            hook_type: HookType::Action,
            description: "Fires during checkout processing".to_string(),
            parameters: vec!["checkout_data".to_string()],
        },
        Hook {
            name: "rustcommerce_after_checkout_validation".to_string(),
            hook_type: HookType::Action,
            description: "Fires after checkout validation".to_string(),
            parameters: vec!["data".to_string(), "errors".to_string()],
        },
        Hook {
            name: "rustcommerce_checkout_order_created".to_string(),
            hook_type: HookType::Action,
            description: "Fires after order is created during checkout".to_string(),
            parameters: vec!["order_id".to_string(), "order".to_string()],
        },

        // Order hooks
        Hook {
            name: "rustcommerce_new_order".to_string(),
            hook_type: HookType::Action,
            description: "Fires when a new order is created".to_string(),
            parameters: vec!["order_id".to_string(), "order".to_string()],
        },
        Hook {
            name: "rustcommerce_order_status_changed".to_string(),
            hook_type: HookType::Action,
            description: "Fires when order status changes".to_string(),
            parameters: vec!["order_id".to_string(), "old_status".to_string(), "new_status".to_string()],
        },
        Hook {
            name: "rustcommerce_payment_complete".to_string(),
            hook_type: HookType::Action,
            description: "Fires when payment is complete".to_string(),
            parameters: vec!["order_id".to_string()],
        },
        Hook {
            name: "rustcommerce_order_refunded".to_string(),
            hook_type: HookType::Action,
            description: "Fires when an order is refunded".to_string(),
            parameters: vec!["order_id".to_string(), "refund_id".to_string()],
        },

        // Customer hooks
        Hook {
            name: "rustcommerce_created_customer".to_string(),
            hook_type: HookType::Action,
            description: "Fires when a customer is created".to_string(),
            parameters: vec!["customer_id".to_string()],
        },
        Hook {
            name: "rustcommerce_customer_save_address".to_string(),
            hook_type: HookType::Action,
            description: "Fires when customer saves an address".to_string(),
            parameters: vec!["customer_id".to_string(), "address_type".to_string()],
        },

        // Coupon hooks
        Hook {
            name: "rustcommerce_applied_coupon".to_string(),
            hook_type: HookType::Action,
            description: "Fires when a coupon is applied".to_string(),
            parameters: vec!["coupon_code".to_string()],
        },
        Hook {
            name: "rustcommerce_removed_coupon".to_string(),
            hook_type: HookType::Action,
            description: "Fires when a coupon is removed".to_string(),
            parameters: vec!["coupon_code".to_string()],
        },
        Hook {
            name: "rustcommerce_coupon_discount_amount".to_string(),
            hook_type: HookType::Filter,
            description: "Filter the coupon discount amount".to_string(),
            parameters: vec!["discount".to_string(), "coupon".to_string(), "cart".to_string()],
        },

        // Shipping hooks
        Hook {
            name: "rustcommerce_shipping_method_chosen".to_string(),
            hook_type: HookType::Action,
            description: "Fires when a shipping method is chosen".to_string(),
            parameters: vec!["method_id".to_string()],
        },
        Hook {
            name: "rustcommerce_package_rates".to_string(),
            hook_type: HookType::Filter,
            description: "Filter the available shipping rates".to_string(),
            parameters: vec!["rates".to_string(), "package".to_string()],
        },

        // Tax hooks
        Hook {
            name: "rustcommerce_get_tax_location".to_string(),
            hook_type: HookType::Filter,
            description: "Filter the tax location".to_string(),
            parameters: vec!["location".to_string(), "customer".to_string()],
        },
        Hook {
            name: "rustcommerce_calc_tax".to_string(),
            hook_type: HookType::Filter,
            description: "Filter calculated taxes".to_string(),
            parameters: vec!["taxes".to_string(), "price".to_string(), "rates".to_string()],
        },

        // Payment hooks
        Hook {
            name: "rustcommerce_before_payment_process".to_string(),
            hook_type: HookType::Action,
            description: "Fires before payment is processed".to_string(),
            parameters: vec!["order_id".to_string()],
        },
        Hook {
            name: "rustcommerce_payment_successful".to_string(),
            hook_type: HookType::Action,
            description: "Fires when payment is successful".to_string(),
            parameters: vec!["order_id".to_string(), "transaction_id".to_string()],
        },
        Hook {
            name: "rustcommerce_payment_failed".to_string(),
            hook_type: HookType::Action,
            description: "Fires when payment fails".to_string(),
            parameters: vec!["order_id".to_string(), "error".to_string()],
        },

        // Inventory hooks
        Hook {
            name: "rustcommerce_reduce_stock_levels".to_string(),
            hook_type: HookType::Action,
            description: "Fires when stock levels are reduced".to_string(),
            parameters: vec!["order".to_string()],
        },
        Hook {
            name: "rustcommerce_restore_stock_levels".to_string(),
            hook_type: HookType::Action,
            description: "Fires when stock levels are restored".to_string(),
            parameters: vec!["order".to_string()],
        },
        Hook {
            name: "rustcommerce_low_stock".to_string(),
            hook_type: HookType::Action,
            description: "Fires when a product has low stock".to_string(),
            parameters: vec!["product".to_string()],
        },
        Hook {
            name: "rustcommerce_no_stock".to_string(),
            hook_type: HookType::Action,
            description: "Fires when a product is out of stock".to_string(),
            parameters: vec!["product".to_string()],
        },

        // Email hooks
        Hook {
            name: "rustcommerce_email_order_details".to_string(),
            hook_type: HookType::Action,
            description: "Add content to order email".to_string(),
            parameters: vec!["order".to_string(), "sent_to_admin".to_string()],
        },
    ]
}

/// Get hooks by type
pub fn get_hooks_by_type(hook_type: HookType) -> Vec<Hook> {
    get_hooks().into_iter()
        .filter(|h| h.hook_type == hook_type)
        .collect()
}
