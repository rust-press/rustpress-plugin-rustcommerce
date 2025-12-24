//! RustCommerce Widgets
//!
//! Widgets for displaying e-commerce content in sidebars.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Widget definition
#[derive(Debug, Clone)]
pub struct Widget {
    pub id: String,
    pub name: String,
    pub description: String,
    pub fields: Vec<WidgetField>,
}

/// Widget field
#[derive(Debug, Clone)]
pub struct WidgetField {
    pub name: String,
    pub label: String,
    pub field_type: WidgetFieldType,
    pub default: Option<String>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy)]
pub enum WidgetFieldType {
    Text,
    Number,
    Checkbox,
    Select,
    Textarea,
}

/// Get all RustCommerce widgets
pub fn get_widgets() -> Vec<Widget> {
    vec![
        Widget {
            id: "rc_products".to_string(),
            name: "Products".to_string(),
            description: "Display a list of products".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Products".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "number".to_string(),
                    label: "Number of products".to_string(),
                    field_type: WidgetFieldType::Number,
                    default: Some("5".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "show".to_string(),
                    label: "Show".to_string(),
                    field_type: WidgetFieldType::Select,
                    default: Some("all".to_string()),
                    options: vec![
                        ("all".to_string(), "All products".to_string()),
                        ("featured".to_string(), "Featured products".to_string()),
                        ("onsale".to_string(), "On-sale products".to_string()),
                    ],
                },
                WidgetField {
                    name: "orderby".to_string(),
                    label: "Order by".to_string(),
                    field_type: WidgetFieldType::Select,
                    default: Some("date".to_string()),
                    options: vec![
                        ("date".to_string(), "Date".to_string()),
                        ("price".to_string(), "Price".to_string()),
                        ("sales".to_string(), "Sales".to_string()),
                        ("rating".to_string(), "Rating".to_string()),
                    ],
                },
                WidgetField {
                    name: "order".to_string(),
                    label: "Order".to_string(),
                    field_type: WidgetFieldType::Select,
                    default: Some("DESC".to_string()),
                    options: vec![
                        ("ASC".to_string(), "Ascending".to_string()),
                        ("DESC".to_string(), "Descending".to_string()),
                    ],
                },
                WidgetField {
                    name: "hide_free".to_string(),
                    label: "Hide free products".to_string(),
                    field_type: WidgetFieldType::Checkbox,
                    default: None,
                    options: vec![],
                },
                WidgetField {
                    name: "show_hidden".to_string(),
                    label: "Show hidden products".to_string(),
                    field_type: WidgetFieldType::Checkbox,
                    default: None,
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_recently_viewed".to_string(),
            name: "Recently Viewed Products".to_string(),
            description: "Display products the customer has recently viewed".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Recently Viewed".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "number".to_string(),
                    label: "Number of products".to_string(),
                    field_type: WidgetFieldType::Number,
                    default: Some("5".to_string()),
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_product_categories".to_string(),
            name: "Product Categories".to_string(),
            description: "Display product categories".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Product Categories".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "orderby".to_string(),
                    label: "Order by".to_string(),
                    field_type: WidgetFieldType::Select,
                    default: Some("name".to_string()),
                    options: vec![
                        ("name".to_string(), "Name".to_string()),
                        ("order".to_string(), "Category order".to_string()),
                        ("count".to_string(), "Count".to_string()),
                    ],
                },
                WidgetField {
                    name: "dropdown".to_string(),
                    label: "Show as dropdown".to_string(),
                    field_type: WidgetFieldType::Checkbox,
                    default: None,
                    options: vec![],
                },
                WidgetField {
                    name: "count".to_string(),
                    label: "Show product counts".to_string(),
                    field_type: WidgetFieldType::Checkbox,
                    default: Some("yes".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "hierarchical".to_string(),
                    label: "Show hierarchy".to_string(),
                    field_type: WidgetFieldType::Checkbox,
                    default: Some("yes".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "hide_empty".to_string(),
                    label: "Hide empty categories".to_string(),
                    field_type: WidgetFieldType::Checkbox,
                    default: Some("yes".to_string()),
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_product_tags".to_string(),
            name: "Product Tags".to_string(),
            description: "Display product tags as a cloud".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Product Tags".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "number".to_string(),
                    label: "Number of tags".to_string(),
                    field_type: WidgetFieldType::Number,
                    default: Some("20".to_string()),
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_cart".to_string(),
            name: "Cart".to_string(),
            description: "Display shopping cart summary".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Cart".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "hide_if_empty".to_string(),
                    label: "Hide if cart is empty".to_string(),
                    field_type: WidgetFieldType::Checkbox,
                    default: None,
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_layered_nav".to_string(),
            name: "Filter Products by Attribute".to_string(),
            description: "Display a layered navigation filter".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Filter by".to_string()),
                    options: vec![],
                },
                WidgetField {
                    name: "attribute".to_string(),
                    label: "Attribute".to_string(),
                    field_type: WidgetFieldType::Select,
                    default: None,
                    options: vec![], // Would be populated dynamically
                },
                WidgetField {
                    name: "display_type".to_string(),
                    label: "Display type".to_string(),
                    field_type: WidgetFieldType::Select,
                    default: Some("list".to_string()),
                    options: vec![
                        ("list".to_string(), "List".to_string()),
                        ("dropdown".to_string(), "Dropdown".to_string()),
                    ],
                },
                WidgetField {
                    name: "query_type".to_string(),
                    label: "Query type".to_string(),
                    field_type: WidgetFieldType::Select,
                    default: Some("and".to_string()),
                    options: vec![
                        ("and".to_string(), "AND".to_string()),
                        ("or".to_string(), "OR".to_string()),
                    ],
                },
            ],
        },
        Widget {
            id: "rc_layered_nav_filters".to_string(),
            name: "Active Filters".to_string(),
            description: "Display currently active product filters".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Active Filters".to_string()),
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_price_filter".to_string(),
            name: "Filter Products by Price".to_string(),
            description: "Display a price filter slider".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Filter by price".to_string()),
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_rating_filter".to_string(),
            name: "Filter Products by Rating".to_string(),
            description: "Display a rating filter".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: Some("Filter by rating".to_string()),
                    options: vec![],
                },
            ],
        },
        Widget {
            id: "rc_product_search".to_string(),
            name: "Product Search".to_string(),
            description: "A search form for your store".to_string(),
            fields: vec![
                WidgetField {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                    field_type: WidgetFieldType::Text,
                    default: None,
                    options: vec![],
                },
            ],
        },
    ]
}

/// Render widget output
pub fn render_widget(id: &str, settings: &HashMap<String, String>) -> String {
    match id {
        "rc_cart" => render_cart_widget(settings),
        "rc_product_search" => render_search_widget(settings),
        _ => format!("<!-- Widget {} not implemented -->", id),
    }
}

fn render_cart_widget(settings: &HashMap<String, String>) -> String {
    let title = settings.get("title").cloned().unwrap_or_else(|| "Cart".to_string());

    format!(
        r#"<div class="rc-widget rc-cart-widget">
            <h3 class="widget-title">{}</h3>
            <div class="rc-cart-widget-content" data-rc-cart-widget></div>
        </div>"#,
        title
    )
}

fn render_search_widget(settings: &HashMap<String, String>) -> String {
    let title = settings.get("title").cloned();

    let title_html = title.map(|t| format!(r#"<h3 class="widget-title">{}</h3>"#, t))
        .unwrap_or_default();

    format!(
        r#"<div class="rc-widget rc-search-widget">
            {}
            <form class="rc-product-search" action="/shop" method="get">
                <input type="search" name="s" placeholder="Search products..." />
                <input type="hidden" name="post_type" value="product" />
                <button type="submit">Search</button>
            </form>
        </div>"#,
        title_html
    )
}
