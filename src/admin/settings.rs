//! Admin Settings
//!
//! Settings management admin functionality.

use serde::{Deserialize, Serialize};

/// Settings tab configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsTab {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub sections: Vec<SettingsSection>,
}

/// Settings section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSection {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<SettingsField>,
}

/// Settings field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsField {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub field_type: SettingsFieldType,
    pub default: Option<String>,
    pub options: Vec<(String, String)>,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsFieldType {
    Text,
    Textarea,
    Select,
    Multiselect,
    Checkbox,
    Radio,
    Number,
    Password,
    Email,
    Url,
    Color,
    Image,
    File,
}

/// Get settings tabs
pub fn get_settings_tabs() -> Vec<SettingsTab> {
    vec![
        SettingsTab {
            id: "general".to_string(),
            title: "General".to_string(),
            icon: Some("settings".to_string()),
            sections: vec![
                SettingsSection {
                    id: "store-address".to_string(),
                    title: "Store Address".to_string(),
                    description: Some("This is where your business is located.".to_string()),
                    fields: vec![
                        SettingsField {
                            id: "store_address".to_string(),
                            title: "Address line 1".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Text,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "store_address_2".to_string(),
                            title: "Address line 2".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Text,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "store_city".to_string(),
                            title: "City".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Text,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "default_country".to_string(),
                            title: "Country / State".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Select,
                            default: Some("US".to_string()),
                            options: vec![],
                            required: true,
                        },
                        SettingsField {
                            id: "store_postcode".to_string(),
                            title: "Postcode / ZIP".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Text,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                    ],
                },
                SettingsSection {
                    id: "currency-options".to_string(),
                    title: "Currency Options".to_string(),
                    description: Some("The following options affect how prices are displayed.".to_string()),
                    fields: vec![
                        SettingsField {
                            id: "currency".to_string(),
                            title: "Currency".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Select,
                            default: Some("USD".to_string()),
                            options: vec![],
                            required: true,
                        },
                        SettingsField {
                            id: "currency_pos".to_string(),
                            title: "Currency position".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Select,
                            default: Some("left".to_string()),
                            options: vec![
                                ("left".to_string(), "Left ($99.99)".to_string()),
                                ("right".to_string(), "Right (99.99$)".to_string()),
                                ("left_space".to_string(), "Left with space ($ 99.99)".to_string()),
                                ("right_space".to_string(), "Right with space (99.99 $)".to_string()),
                            ],
                            required: true,
                        },
                        SettingsField {
                            id: "thousand_sep".to_string(),
                            title: "Thousand separator".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Text,
                            default: Some(",".to_string()),
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "decimal_sep".to_string(),
                            title: "Decimal separator".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Text,
                            default: Some(".".to_string()),
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "num_decimals".to_string(),
                            title: "Number of decimals".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Number,
                            default: Some("2".to_string()),
                            options: vec![],
                            required: true,
                        },
                    ],
                },
            ],
        },
        SettingsTab {
            id: "products".to_string(),
            title: "Products".to_string(),
            icon: Some("package".to_string()),
            sections: vec![
                SettingsSection {
                    id: "shop-pages".to_string(),
                    title: "Shop Pages".to_string(),
                    description: None,
                    fields: vec![
                        SettingsField {
                            id: "shop_page_id".to_string(),
                            title: "Shop page".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Select,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "cart_page_id".to_string(),
                            title: "Cart page".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Select,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "checkout_page_id".to_string(),
                            title: "Checkout page".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Select,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "myaccount_page_id".to_string(),
                            title: "My account page".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Select,
                            default: None,
                            options: vec![],
                            required: false,
                        },
                    ],
                },
                SettingsSection {
                    id: "inventory".to_string(),
                    title: "Inventory".to_string(),
                    description: None,
                    fields: vec![
                        SettingsField {
                            id: "manage_stock".to_string(),
                            title: "Manage stock".to_string(),
                            description: Some("Enable stock management".to_string()),
                            field_type: SettingsFieldType::Checkbox,
                            default: Some("yes".to_string()),
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "notify_low_stock".to_string(),
                            title: "Low stock notifications".to_string(),
                            description: Some("Enable low stock notifications".to_string()),
                            field_type: SettingsFieldType::Checkbox,
                            default: Some("yes".to_string()),
                            options: vec![],
                            required: false,
                        },
                        SettingsField {
                            id: "low_stock_amount".to_string(),
                            title: "Low stock threshold".to_string(),
                            description: None,
                            field_type: SettingsFieldType::Number,
                            default: Some("2".to_string()),
                            options: vec![],
                            required: false,
                        },
                    ],
                },
            ],
        },
        SettingsTab {
            id: "tax".to_string(),
            title: "Tax".to_string(),
            icon: Some("percent".to_string()),
            sections: vec![],
        },
        SettingsTab {
            id: "shipping".to_string(),
            title: "Shipping".to_string(),
            icon: Some("truck".to_string()),
            sections: vec![],
        },
        SettingsTab {
            id: "payments".to_string(),
            title: "Payments".to_string(),
            icon: Some("credit-card".to_string()),
            sections: vec![],
        },
        SettingsTab {
            id: "accounts".to_string(),
            title: "Accounts & Privacy".to_string(),
            icon: Some("user".to_string()),
            sections: vec![],
        },
        SettingsTab {
            id: "emails".to_string(),
            title: "Emails".to_string(),
            icon: Some("mail".to_string()),
            sections: vec![],
        },
        SettingsTab {
            id: "advanced".to_string(),
            title: "Advanced".to_string(),
            icon: Some("sliders".to_string()),
            sections: vec![],
        },
    ]
}
