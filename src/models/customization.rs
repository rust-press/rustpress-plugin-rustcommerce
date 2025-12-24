//! Product Customization Models
//!
//! Self-contained product personalization and customization options.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Product customization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCustomization {
    pub id: Uuid,
    pub product_id: Uuid,
    pub is_enabled: bool,
    pub fields: Vec<CustomizationField>,
    pub preview_enabled: bool,
    pub preview_type: PreviewType,
    pub base_image_id: Option<Uuid>,
    pub pricing_type: CustomizationPricingType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewType {
    None,
    Image,
    ThreeD,
    Live,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomizationPricingType {
    Free,
    FixedTotal,
    PerField,
}

/// Customization field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationField {
    pub id: Uuid,
    pub customization_id: Uuid,
    pub field_type: FieldType,
    pub label: String,
    pub placeholder: Option<String>,
    pub description: Option<String>,
    pub required: bool,
    pub sort_order: i32,

    // Validation
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub pattern: Option<String>,
    pub allowed_values: Option<Vec<String>>,

    // Options (for select, radio, checkbox)
    pub options: Option<Vec<FieldOption>>,

    // File upload settings
    pub allowed_file_types: Option<Vec<String>>,
    pub max_file_size_mb: Option<i32>,

    // Pricing
    pub price_adjustment: Option<Decimal>,
    pub price_type: Option<PriceAdjustmentType>,

    // Display conditions
    pub display_condition: Option<DisplayCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Textarea,
    Number,
    Select,
    Radio,
    Checkbox,
    ColorPicker,
    DatePicker,
    TimePicker,
    FileUpload,
    ImageUpload,
    Dropdown,
    MultiSelect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
    pub price_adjustment: Option<Decimal>,
    pub image_url: Option<String>,
    pub color_code: Option<String>,
    pub is_default: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceAdjustmentType {
    Fixed,
    Percentage,
    PerCharacter,
    PerLine,
}

/// Display condition for conditional fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayCondition {
    pub field_id: Uuid,
    pub operator: ConditionOperator,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    IsEmpty,
    IsNotEmpty,
}

/// Custom product template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationTemplate {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<CustomizationField>,
    pub is_default: bool,
    pub category_ids: Option<Vec<Uuid>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Customer customization (saved values)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCustomization {
    pub id: Uuid,
    pub order_item_id: Option<Uuid>,
    pub cart_item_id: Option<Uuid>,
    pub product_id: Uuid,
    pub customization_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub values: Vec<CustomizationValue>,
    pub total_price_adjustment: Decimal,
    pub preview_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationValue {
    pub field_id: Uuid,
    pub field_label: String,
    pub value: String,
    pub display_value: String, // Formatted for display
    pub file_url: Option<String>,
    pub price_adjustment: Decimal,
}

/// Print area (for print-on-demand)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintArea {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub position_x: f32,
    pub position_y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub max_colors: Option<i32>,
    pub print_method: PrintMethod,
    pub additional_cost: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrintMethod {
    ScreenPrint,
    Dtg,
    Embroidery,
    HeatTransfer,
    Sublimation,
    Engraving,
    Laser,
}

/// Design asset (uploaded by customer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignAsset {
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub file_name: String,
    pub file_url: String,
    pub file_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub status: AssetStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    Uploading,
    Processing,
    Ready,
    Failed,
    Expired,
}

/// Design/customization preview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignPreview {
    pub id: Uuid,
    pub product_id: Uuid,
    pub customization_values: HashMap<String, String>,
    pub preview_images: Vec<PreviewImage>,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewImage {
    pub view: String, // front, back, side, etc.
    pub url: String,
    pub thumbnail_url: Option<String>,
}

/// Customization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationSettings {
    pub enabled: bool,
    pub max_file_size_mb: i32,
    pub allowed_file_types: Vec<String>,
    pub auto_approve_text: bool,
    pub require_design_approval: bool,
    pub preview_generation_enabled: bool,
    pub allow_save_for_later: bool,
    pub file_expiry_days: i32,
}

impl Default for CustomizationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_file_size_mb: 10,
            allowed_file_types: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "svg".to_string(),
                "pdf".to_string(),
            ],
            auto_approve_text: true,
            require_design_approval: false,
            preview_generation_enabled: true,
            allow_save_for_later: true,
            file_expiry_days: 30,
        }
    }
}

impl ProductCustomization {
    /// Calculate total price adjustment
    pub fn calculate_price_adjustment(&self, values: &[CustomizationValue]) -> Decimal {
        values.iter().map(|v| v.price_adjustment).sum()
    }

    /// Validate customization values
    pub fn validate(&self, values: &[CustomizationValue]) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for field in &self.fields {
            let value = values.iter().find(|v| v.field_id == field.id);

            // Check required fields
            if field.required {
                if value.is_none() || value.map_or(true, |v| v.value.is_empty()) {
                    errors.push(ValidationError {
                        field_id: field.id,
                        field_label: field.label.clone(),
                        error_type: ValidationErrorType::Required,
                        message: format!("{} is required", field.label),
                    });
                    continue;
                }
            }

            if let Some(val) = value {
                // Check length constraints
                if let Some(min) = field.min_length {
                    if val.value.len() < min as usize {
                        errors.push(ValidationError {
                            field_id: field.id,
                            field_label: field.label.clone(),
                            error_type: ValidationErrorType::TooShort,
                            message: format!("{} must be at least {} characters", field.label, min),
                        });
                    }
                }

                if let Some(max) = field.max_length {
                    if val.value.len() > max as usize {
                        errors.push(ValidationError {
                            field_id: field.id,
                            field_label: field.label.clone(),
                            error_type: ValidationErrorType::TooLong,
                            message: format!("{} must be at most {} characters", field.label, max),
                        });
                    }
                }

                // Check allowed values
                if let Some(ref allowed) = field.allowed_values {
                    if !allowed.contains(&val.value) {
                        errors.push(ValidationError {
                            field_id: field.id,
                            field_label: field.label.clone(),
                            error_type: ValidationErrorType::InvalidValue,
                            message: format!("Invalid value for {}", field.label),
                        });
                    }
                }
            }
        }

        errors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field_id: Uuid,
    pub field_label: String,
    pub error_type: ValidationErrorType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationErrorType {
    Required,
    TooShort,
    TooLong,
    InvalidValue,
    InvalidFormat,
    InvalidFileType,
    FileTooLarge,
}

/// Submit customization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitCustomizationRequest {
    pub product_id: Uuid,
    pub values: Vec<CustomizationValueInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationValueInput {
    pub field_id: Uuid,
    pub value: String,
    pub file_id: Option<Uuid>,
}

/// Customization response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationResponse {
    pub customization_id: Uuid,
    pub values: Vec<CustomizationValue>,
    pub total_price_adjustment: Decimal,
    pub preview_url: Option<String>,
    pub validation_errors: Vec<ValidationError>,
}
