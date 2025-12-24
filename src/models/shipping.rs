//! Shipping Models

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Shipping zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZone {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub zone_order: i32,
    pub locations: Vec<ShippingZoneLocation>,
    pub methods: Vec<ShippingZoneMethod>,
    pub created_at: DateTime<Utc>,
}

/// Shipping zone location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZoneLocation {
    pub id: Uuid,
    pub zone_id: Uuid,
    pub location_code: String,
    pub location_type: LocationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    Country,
    State,
    Postcode,
    Continent,
}

/// Shipping zone method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZoneMethod {
    pub id: Uuid,
    pub zone_id: Uuid,
    pub method_id: String,
    pub method_order: i32,
    pub is_enabled: bool,
    pub settings: ShippingMethodSettings,
}

/// Shipping method configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethod {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub method_id: String,
    pub method_title: String,
    pub method_description: Option<String>,
    pub settings: ShippingMethodSettings,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Shipping method settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingMethodSettings {
    // Flat rate settings
    pub cost: Option<Decimal>,
    pub cost_per_item: Option<Decimal>,
    pub cost_per_weight_unit: Option<Decimal>,

    // Free shipping settings
    pub min_amount: Option<Decimal>,
    pub requires_coupon: Option<bool>,

    // Class costs
    pub class_costs: std::collections::HashMap<String, Decimal>,
    pub no_class_cost: Option<Decimal>,

    // Calculation type
    pub calc_type: Option<ShippingCalcType>,

    // Tax
    pub tax_status: Option<ShippingTaxStatus>,

    // Local pickup
    pub pickup_location: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShippingCalcType {
    #[default]
    PerOrder,
    PerClass,
    PerItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShippingTaxStatus {
    #[default]
    Taxable,
    None,
}

/// Shipping class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClass {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Calculated shipping rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedShippingRate {
    pub id: String,
    pub method_id: String,
    pub instance_id: String,
    pub label: String,
    pub cost: Decimal,
    pub taxes: std::collections::HashMap<Uuid, Decimal>,
    pub meta: std::collections::HashMap<String, String>,
    pub package_id: String,
}

/// Shipping package (for split shipping)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingPackage {
    pub id: String,
    pub contents_cost: Decimal,
    pub contents_weight: Decimal,
    pub destination: ShippingDestination,
    pub items: Vec<PackageItem>,
}

/// Shipping destination
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingDestination {
    pub country: String,
    pub state: String,
    pub postcode: String,
    pub city: String,
}

/// Package item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageItem {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: i32,
    pub weight: Option<Decimal>,
    pub shipping_class_id: Option<Uuid>,
}
