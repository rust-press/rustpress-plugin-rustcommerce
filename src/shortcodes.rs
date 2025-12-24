//! RustCommerce Shortcodes
//!
//! Shortcodes for embedding e-commerce content in pages and posts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shortcode definition
#[derive(Debug, Clone)]
pub struct Shortcode {
    pub tag: String,
    pub description: String,
    pub attributes: Vec<ShortcodeAttribute>,
    pub example: String,
}

/// Shortcode attribute
#[derive(Debug, Clone)]
pub struct ShortcodeAttribute {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
    pub required: bool,
}

/// Get all RustCommerce shortcodes
pub fn get_shortcodes() -> Vec<Shortcode> {
    vec![
        // Product shortcodes
        Shortcode {
            tag: "rc_product".to_string(),
            description: "Display a single product".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "id".to_string(),
                    description: "Product ID to display".to_string(),
                    default: None,
                    required: true,
                },
            ],
            example: r#"[rc_product id="123"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_products".to_string(),
            description: "Display multiple products".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "ids".to_string(),
                    description: "Comma-separated product IDs".to_string(),
                    default: None,
                    required: false,
                },
                ShortcodeAttribute {
                    name: "limit".to_string(),
                    description: "Number of products to show".to_string(),
                    default: Some("12".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "columns".to_string(),
                    description: "Number of columns".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "orderby".to_string(),
                    description: "Order by field (date, title, price, popularity)".to_string(),
                    default: Some("date".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "order".to_string(),
                    description: "Sort order (ASC or DESC)".to_string(),
                    default: Some("DESC".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "category".to_string(),
                    description: "Category slug".to_string(),
                    default: None,
                    required: false,
                },
                ShortcodeAttribute {
                    name: "tag".to_string(),
                    description: "Tag slug".to_string(),
                    default: None,
                    required: false,
                },
            ],
            example: r#"[rc_products limit="8" columns="4" orderby="popularity"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_recent_products".to_string(),
            description: "Display recently added products".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "limit".to_string(),
                    description: "Number of products".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "columns".to_string(),
                    description: "Number of columns".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
            ],
            example: r#"[rc_recent_products limit="8"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_sale_products".to_string(),
            description: "Display products on sale".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "limit".to_string(),
                    description: "Number of products".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "columns".to_string(),
                    description: "Number of columns".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
            ],
            example: r#"[rc_sale_products limit="8"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_featured_products".to_string(),
            description: "Display featured products".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "limit".to_string(),
                    description: "Number of products".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "columns".to_string(),
                    description: "Number of columns".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
            ],
            example: r#"[rc_featured_products limit="4"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_best_selling_products".to_string(),
            description: "Display best selling products".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "limit".to_string(),
                    description: "Number of products".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "columns".to_string(),
                    description: "Number of columns".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
            ],
            example: r#"[rc_best_selling_products limit="4"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_top_rated_products".to_string(),
            description: "Display top rated products".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "limit".to_string(),
                    description: "Number of products".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "columns".to_string(),
                    description: "Number of columns".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
            ],
            example: r#"[rc_top_rated_products limit="4"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_product_categories".to_string(),
            description: "Display product categories".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "number".to_string(),
                    description: "Number of categories".to_string(),
                    default: None,
                    required: false,
                },
                ShortcodeAttribute {
                    name: "columns".to_string(),
                    description: "Number of columns".to_string(),
                    default: Some("4".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "parent".to_string(),
                    description: "Parent category ID".to_string(),
                    default: None,
                    required: false,
                },
                ShortcodeAttribute {
                    name: "hide_empty".to_string(),
                    description: "Hide empty categories".to_string(),
                    default: Some("true".to_string()),
                    required: false,
                },
            ],
            example: r#"[rc_product_categories columns="3"]"#.to_string(),
        },

        // Cart shortcodes
        Shortcode {
            tag: "rc_cart".to_string(),
            description: "Display the shopping cart".to_string(),
            attributes: vec![],
            example: r#"[rc_cart]"#.to_string(),
        },
        Shortcode {
            tag: "rc_cart_totals".to_string(),
            description: "Display cart totals".to_string(),
            attributes: vec![],
            example: r#"[rc_cart_totals]"#.to_string(),
        },

        // Checkout shortcodes
        Shortcode {
            tag: "rc_checkout".to_string(),
            description: "Display the checkout form".to_string(),
            attributes: vec![],
            example: r#"[rc_checkout]"#.to_string(),
        },
        Shortcode {
            tag: "rc_order_tracking".to_string(),
            description: "Display order tracking form".to_string(),
            attributes: vec![],
            example: r#"[rc_order_tracking]"#.to_string(),
        },

        // Account shortcodes
        Shortcode {
            tag: "rc_my_account".to_string(),
            description: "Display the customer account page".to_string(),
            attributes: vec![],
            example: r#"[rc_my_account]"#.to_string(),
        },

        // Other shortcodes
        Shortcode {
            tag: "rc_add_to_cart".to_string(),
            description: "Display add to cart button for a product".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "id".to_string(),
                    description: "Product ID".to_string(),
                    default: None,
                    required: true,
                },
                ShortcodeAttribute {
                    name: "quantity".to_string(),
                    description: "Default quantity".to_string(),
                    default: Some("1".to_string()),
                    required: false,
                },
                ShortcodeAttribute {
                    name: "show_price".to_string(),
                    description: "Show price".to_string(),
                    default: Some("true".to_string()),
                    required: false,
                },
            ],
            example: r#"[rc_add_to_cart id="123"]"#.to_string(),
        },
        Shortcode {
            tag: "rc_add_to_cart_url".to_string(),
            description: "Get add to cart URL for a product".to_string(),
            attributes: vec![
                ShortcodeAttribute {
                    name: "id".to_string(),
                    description: "Product ID".to_string(),
                    default: None,
                    required: true,
                },
            ],
            example: r#"[rc_add_to_cart_url id="123"]"#.to_string(),
        },
    ]
}

/// Render shortcode output
pub fn render_shortcode(tag: &str, attributes: &HashMap<String, String>) -> String {
    match tag {
        "rc_cart" => render_cart(),
        "rc_checkout" => render_checkout(),
        "rc_my_account" => render_my_account(),
        _ => format!("<!-- Unknown shortcode: {} -->", tag),
    }
}

fn render_cart() -> String {
    r#"<div class="rc-cart" data-rc-cart></div>"#.to_string()
}

fn render_checkout() -> String {
    r#"<div class="rc-checkout" data-rc-checkout></div>"#.to_string()
}

fn render_my_account() -> String {
    r#"<div class="rc-my-account" data-rc-my-account></div>"#.to_string()
}
