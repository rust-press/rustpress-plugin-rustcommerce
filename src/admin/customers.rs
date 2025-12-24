//! Admin Customers
//!
//! Customer management admin functionality.

use serde::{Deserialize, Serialize};

/// Customer list column configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerListColumn {
    pub id: String,
    pub title: String,
    pub sortable: bool,
    pub width: Option<String>,
}

/// Get default customer list columns
pub fn get_customer_columns() -> Vec<CustomerListColumn> {
    vec![
        CustomerListColumn {
            id: "name".to_string(),
            title: "Name".to_string(),
            sortable: true,
            width: None,
        },
        CustomerListColumn {
            id: "email".to_string(),
            title: "Email".to_string(),
            sortable: true,
            width: Some("200px".to_string()),
        },
        CustomerListColumn {
            id: "location".to_string(),
            title: "Location".to_string(),
            sortable: false,
            width: Some("150px".to_string()),
        },
        CustomerListColumn {
            id: "orders".to_string(),
            title: "Orders".to_string(),
            sortable: true,
            width: Some("80px".to_string()),
        },
        CustomerListColumn {
            id: "total_spent".to_string(),
            title: "Total Spent".to_string(),
            sortable: true,
            width: Some("120px".to_string()),
        },
        CustomerListColumn {
            id: "aov".to_string(),
            title: "AOV".to_string(),
            sortable: true,
            width: Some("100px".to_string()),
        },
        CustomerListColumn {
            id: "last_order".to_string(),
            title: "Last Order".to_string(),
            sortable: true,
            width: Some("120px".to_string()),
        },
        CustomerListColumn {
            id: "registered".to_string(),
            title: "Registered".to_string(),
            sortable: true,
            width: Some("120px".to_string()),
        },
        CustomerListColumn {
            id: "actions".to_string(),
            title: "Actions".to_string(),
            sortable: false,
            width: Some("80px".to_string()),
        },
    ]
}

/// Customer bulk actions
pub fn get_customer_bulk_actions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("delete", "Delete"),
        ("export", "Export"),
    ]
}

/// Customer editor tabs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerEditorTab {
    pub id: String,
    pub title: String,
}

/// Get customer editor tabs
pub fn get_customer_editor_tabs() -> Vec<CustomerEditorTab> {
    vec![
        CustomerEditorTab {
            id: "general".to_string(),
            title: "General".to_string(),
        },
        CustomerEditorTab {
            id: "billing".to_string(),
            title: "Billing Address".to_string(),
        },
        CustomerEditorTab {
            id: "shipping".to_string(),
            title: "Shipping Address".to_string(),
        },
        CustomerEditorTab {
            id: "orders".to_string(),
            title: "Orders".to_string(),
        },
        CustomerEditorTab {
            id: "downloads".to_string(),
            title: "Downloads".to_string(),
        },
    ]
}

/// Customer actions
pub fn get_customer_actions() -> Vec<(&'static str, &'static str, &'static str)> {
    // (id, label, icon)
    vec![
        ("view", "View", "eye"),
        ("edit", "Edit", "edit"),
        ("orders", "View Orders", "shopping-bag"),
        ("delete", "Delete", "trash-2"),
    ]
}
