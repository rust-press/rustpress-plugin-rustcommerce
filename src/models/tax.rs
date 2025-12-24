//! Tax Models

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Tax rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub id: Uuid,
    pub site_id: Option<Uuid>,

    // Location
    pub country: String,
    pub state: String,
    pub postcode: String,
    pub city: String,

    // Rate
    pub rate: Decimal,
    pub name: String,

    // Priority
    pub priority: i32,
    pub compound: bool,
    pub shipping: bool,

    // Order
    pub tax_order: i32,

    // Class
    pub tax_class: String,

    pub created_at: DateTime<Utc>,
}

impl TaxRate {
    /// Check if this rate matches a location
    pub fn matches_location(&self, country: &str, state: &str, postcode: &str, city: &str) -> bool {
        // Country must match (or be empty for all)
        if !self.country.is_empty() && self.country != country {
            return false;
        }

        // State must match (or be empty for all)
        if !self.state.is_empty() && self.state != state {
            return false;
        }

        // Postcode matching (supports wildcards)
        if !self.postcode.is_empty() {
            if self.postcode.contains('*') {
                // Wildcard matching
                let pattern = self.postcode.replace('*', "");
                if !postcode.starts_with(&pattern) {
                    return false;
                }
            } else if self.postcode.contains("...") {
                // Range matching (e.g., "12345...12400")
                let parts: Vec<&str> = self.postcode.split("...").collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(end), Ok(pc)) = (
                        parts[0].parse::<i64>(),
                        parts[1].parse::<i64>(),
                        postcode.parse::<i64>()
                    ) {
                        if pc < start || pc > end {
                            return false;
                        }
                    }
                }
            } else if self.postcode != postcode {
                return false;
            }
        }

        // City must match (or be empty for all)
        if !self.city.is_empty() && self.city.to_lowercase() != city.to_lowercase() {
            return false;
        }

        true
    }

    /// Calculate tax for an amount
    pub fn calculate(&self, amount: Decimal) -> Decimal {
        amount * (self.rate / Decimal::from(100))
    }
}

/// Tax class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxClass {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Tax rate location (for additional targeting)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRateLocation {
    pub id: Uuid,
    pub tax_rate_id: Uuid,
    pub location_code: String,
    pub location_type: TaxLocationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxLocationType {
    Postcode,
    City,
}

/// Calculated tax line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedTax {
    pub rate_id: Uuid,
    pub rate_code: String,
    pub label: String,
    pub rate: Decimal,
    pub compound: bool,
    pub tax_amount: Decimal,
    pub shipping_tax_amount: Decimal,
}

/// Tax calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCalculationResult {
    pub taxes: Vec<CalculatedTax>,
    pub total_tax: Decimal,
    pub total_shipping_tax: Decimal,
}

/// Tax location for calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxLocation {
    pub country: String,
    pub state: String,
    pub postcode: String,
    pub city: String,
}

impl TaxLocation {
    pub fn new(country: &str, state: &str, postcode: &str, city: &str) -> Self {
        Self {
            country: country.to_string(),
            state: state.to_string(),
            postcode: postcode.to_string(),
            city: city.to_string(),
        }
    }
}

// =============================================================================
// DTOs for API
// =============================================================================

/// Request to create a tax rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaxRateRequest {
    pub country: Option<String>,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub city: Option<String>,
    pub rate: Decimal,
    pub name: String,
    pub priority: Option<i32>,
    pub compound: Option<bool>,
    pub shipping: Option<bool>,
    pub tax_class: Option<String>,
}

/// Request to update a tax rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaxRateRequest {
    pub country: Option<String>,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub city: Option<String>,
    pub rate: Option<Decimal>,
    pub name: Option<String>,
    pub priority: Option<i32>,
    pub compound: Option<bool>,
    pub shipping: Option<bool>,
    pub tax_class: Option<String>,
}

/// Tax rate filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaxRateFilter {
    pub country: Option<String>,
    pub state: Option<String>,
    pub tax_class: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}
