//! Admin Products
//!
//! Product management admin functionality.

use serde::{Deserialize, Serialize};

/// Product list column configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListColumn {
    pub id: String,
    pub title: String,
    pub sortable: bool,
    pub width: Option<String>,
}

/// Get default product list columns
pub fn get_product_columns() -> Vec<ProductListColumn> {
    vec![
        ProductListColumn {
            id: "image".to_string(),
            title: "Image".to_string(),
            sortable: false,
            width: Some("52px".to_string()),
        },
        ProductListColumn {
            id: "name".to_string(),
            title: "Name".to_string(),
            sortable: true,
            width: None,
        },
        ProductListColumn {
            id: "sku".to_string(),
            title: "SKU".to_string(),
            sortable: true,
            width: Some("100px".to_string()),
        },
        ProductListColumn {
            id: "stock".to_string(),
            title: "Stock".to_string(),
            sortable: true,
            width: Some("100px".to_string()),
        },
        ProductListColumn {
            id: "price".to_string(),
            title: "Price".to_string(),
            sortable: true,
            width: Some("120px".to_string()),
        },
        ProductListColumn {
            id: "categories".to_string(),
            title: "Categories".to_string(),
            sortable: false,
            width: Some("150px".to_string()),
        },
        ProductListColumn {
            id: "tags".to_string(),
            title: "Tags".to_string(),
            sortable: false,
            width: Some("120px".to_string()),
        },
        ProductListColumn {
            id: "date".to_string(),
            title: "Date".to_string(),
            sortable: true,
            width: Some("120px".to_string()),
        },
    ]
}

/// Product bulk actions
pub fn get_product_bulk_actions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("edit", "Edit"),
        ("delete", "Move to Trash"),
        ("set_regular_price", "Set Regular Price"),
        ("increase_regular_price", "Increase Regular Price"),
        ("decrease_regular_price", "Decrease Regular Price"),
        ("set_sale_price", "Set Sale Price"),
        ("set_stock_status_instock", "Set Stock Status: In Stock"),
        ("set_stock_status_outofstock", "Set Stock Status: Out of Stock"),
    ]
}

/// Product editor tabs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductEditorTab {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
}

/// Get product editor tabs
pub fn get_product_editor_tabs() -> Vec<ProductEditorTab> {
    vec![
        ProductEditorTab {
            id: "general".to_string(),
            title: "General".to_string(),
            icon: Some("settings".to_string()),
        },
        ProductEditorTab {
            id: "inventory".to_string(),
            title: "Inventory".to_string(),
            icon: Some("package".to_string()),
        },
        ProductEditorTab {
            id: "shipping".to_string(),
            title: "Shipping".to_string(),
            icon: Some("truck".to_string()),
        },
        ProductEditorTab {
            id: "linked".to_string(),
            title: "Linked Products".to_string(),
            icon: Some("link".to_string()),
        },
        ProductEditorTab {
            id: "attributes".to_string(),
            title: "Attributes".to_string(),
            icon: Some("list".to_string()),
        },
        ProductEditorTab {
            id: "variations".to_string(),
            title: "Variations".to_string(),
            icon: Some("layers".to_string()),
        },
        ProductEditorTab {
            id: "advanced".to_string(),
            title: "Advanced".to_string(),
            icon: Some("sliders".to_string()),
        },
    ]
}

/// Product type options
pub fn get_product_types() -> Vec<(&'static str, &'static str)> {
    vec![
        ("simple", "Simple product"),
        ("grouped", "Grouped product"),
        ("external", "External/Affiliate product"),
        ("variable", "Variable product"),
    ]
}

/// Stock status options
pub fn get_stock_statuses() -> Vec<(&'static str, &'static str)> {
    vec![
        ("instock", "In stock"),
        ("outofstock", "Out of stock"),
        ("onbackorder", "On backorder"),
    ]
}

/// Catalog visibility options
pub fn get_visibility_options() -> Vec<(&'static str, &'static str)> {
    vec![
        ("visible", "Shop and search results"),
        ("catalog", "Shop only"),
        ("search", "Search results only"),
        ("hidden", "Hidden"),
    ]
}
