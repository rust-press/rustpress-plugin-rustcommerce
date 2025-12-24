//! Admin Orders
//!
//! Order management admin functionality.

use serde::{Deserialize, Serialize};

/// Order list column configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListColumn {
    pub id: String,
    pub title: String,
    pub sortable: bool,
    pub width: Option<String>,
}

/// Get default order list columns
pub fn get_order_columns() -> Vec<OrderListColumn> {
    vec![
        OrderListColumn {
            id: "order_number".to_string(),
            title: "Order".to_string(),
            sortable: true,
            width: Some("100px".to_string()),
        },
        OrderListColumn {
            id: "date".to_string(),
            title: "Date".to_string(),
            sortable: true,
            width: Some("120px".to_string()),
        },
        OrderListColumn {
            id: "status".to_string(),
            title: "Status".to_string(),
            sortable: true,
            width: Some("120px".to_string()),
        },
        OrderListColumn {
            id: "billing".to_string(),
            title: "Billing".to_string(),
            sortable: false,
            width: None,
        },
        OrderListColumn {
            id: "shipping".to_string(),
            title: "Ship to".to_string(),
            sortable: false,
            width: None,
        },
        OrderListColumn {
            id: "total".to_string(),
            title: "Total".to_string(),
            sortable: true,
            width: Some("100px".to_string()),
        },
        OrderListColumn {
            id: "actions".to_string(),
            title: "Actions".to_string(),
            sortable: false,
            width: Some("80px".to_string()),
        },
    ]
}

/// Order bulk actions
pub fn get_order_bulk_actions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("mark_processing", "Change status to processing"),
        ("mark_on-hold", "Change status to on-hold"),
        ("mark_completed", "Change status to completed"),
        ("mark_cancelled", "Change status to cancelled"),
        ("trash", "Move to Trash"),
    ]
}

/// Order status colors
pub fn get_status_color(status: &str) -> &'static str {
    match status {
        "pending" => "#f39c12",
        "processing" => "#3498db",
        "on-hold" => "#e74c3c",
        "completed" => "#27ae60",
        "cancelled" => "#95a5a6",
        "refunded" => "#9b59b6",
        "failed" => "#e74c3c",
        _ => "#7f8c8d",
    }
}

/// Order action buttons
pub fn get_order_actions() -> Vec<(&'static str, &'static str, &'static str)> {
    // (id, label, icon)
    vec![
        ("view", "View", "eye"),
        ("edit", "Edit", "edit"),
        ("email", "Email invoice", "mail"),
        ("resend", "Resend notifications", "refresh-cw"),
        ("refund", "Refund", "rotate-ccw"),
        ("delete", "Delete", "trash-2"),
    ]
}

/// Order editor sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderEditorSection {
    pub id: String,
    pub title: String,
}

/// Get order editor sections
pub fn get_order_editor_sections() -> Vec<OrderEditorSection> {
    vec![
        OrderEditorSection {
            id: "general".to_string(),
            title: "General".to_string(),
        },
        OrderEditorSection {
            id: "billing".to_string(),
            title: "Billing".to_string(),
        },
        OrderEditorSection {
            id: "shipping".to_string(),
            title: "Shipping".to_string(),
        },
        OrderEditorSection {
            id: "items".to_string(),
            title: "Items".to_string(),
        },
        OrderEditorSection {
            id: "totals".to_string(),
            title: "Totals".to_string(),
        },
        OrderEditorSection {
            id: "notes".to_string(),
            title: "Order Notes".to_string(),
        },
    ]
}

/// Order status options
pub fn get_order_statuses() -> Vec<(&'static str, &'static str)> {
    vec![
        ("pending", "Pending payment"),
        ("processing", "Processing"),
        ("on-hold", "On hold"),
        ("completed", "Completed"),
        ("cancelled", "Cancelled"),
        ("refunded", "Refunded"),
        ("failed", "Failed"),
    ]
}

/// Email templates available for orders
pub fn get_order_email_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("new_order", "New order (admin)"),
        ("cancelled_order", "Cancelled order (admin)"),
        ("failed_order", "Failed order (admin)"),
        ("processing_order", "Processing order"),
        ("completed_order", "Completed order"),
        ("refunded_order", "Refunded order"),
        ("customer_on_hold_order", "Order on-hold"),
        ("customer_invoice", "Customer invoice"),
    ]
}
