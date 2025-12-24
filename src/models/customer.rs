//! Customer Models

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Customer entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub user_id: Option<Uuid>,

    // Basic info
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,

    // Addresses
    pub billing: Address,
    pub shipping: Address,

    // Stats
    pub orders_count: i32,
    pub total_spent: Decimal,
    pub average_order_value: Decimal,

    // Status
    pub is_paying_customer: bool,
    pub last_order_id: Option<Uuid>,
    pub last_order_date: Option<DateTime<Utc>>,

    // Avatar
    pub avatar_url: Option<String>,

    // Metadata
    pub meta: serde_json::Value,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Customer {
    /// Get full name
    pub fn get_full_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.display_name.clone().unwrap_or_else(|| self.email.clone()),
        }
    }

    /// Check if customer has default shipping address
    pub fn has_shipping_address(&self) -> bool {
        !self.shipping.address_1.is_empty()
    }

    /// Check if customer has default billing address
    pub fn has_billing_address(&self) -> bool {
        !self.billing.address_1.is_empty()
    }
}

/// Address structure (used for billing and shipping)
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct Address {
    #[validate(length(max = 255))]
    pub first_name: String,
    #[validate(length(max = 255))]
    pub last_name: String,
    #[validate(length(max = 255))]
    pub company: String,
    #[validate(length(max = 500))]
    pub address_1: String,
    #[validate(length(max = 500))]
    pub address_2: String,
    #[validate(length(max = 255))]
    pub city: String,
    #[validate(length(max = 100))]
    pub state: String,
    #[validate(length(max = 50))]
    pub postcode: String,
    #[validate(length(min = 2, max = 2))]
    pub country: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(max = 50))]
    pub phone: String,
}

impl Address {
    /// Create empty address
    pub fn empty() -> Self {
        Self::default()
    }

    /// Get formatted address string
    pub fn get_formatted(&self) -> String {
        let mut parts = Vec::new();

        let name = format!("{} {}", self.first_name, self.last_name).trim().to_string();
        if !name.is_empty() {
            parts.push(name);
        }

        if !self.company.is_empty() {
            parts.push(self.company.clone());
        }

        if !self.address_1.is_empty() {
            parts.push(self.address_1.clone());
        }

        if !self.address_2.is_empty() {
            parts.push(self.address_2.clone());
        }

        let city_state_zip = format!(
            "{}, {} {}",
            self.city, self.state, self.postcode
        ).trim().to_string();
        if !city_state_zip.is_empty() && city_state_zip != ", " {
            parts.push(city_state_zip);
        }

        if !self.country.is_empty() {
            parts.push(self.country.clone());
        }

        parts.join("\n")
    }

    /// Check if address is complete (has required fields)
    pub fn is_complete(&self) -> bool {
        !self.first_name.is_empty()
            && !self.last_name.is_empty()
            && !self.address_1.is_empty()
            && !self.city.is_empty()
            && !self.postcode.is_empty()
            && !self.country.is_empty()
    }
}

/// Customer session (for guest carts)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSession {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub session_key: String,
    pub customer_id: Option<Uuid>,
    pub session_value: serde_json::Value,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// DTOs for API
// =============================================================================

/// Request to create a customer
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCustomerRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(max = 255))]
    pub first_name: Option<String>,
    #[validate(length(max = 255))]
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
    pub billing: Option<Address>,
    pub shipping: Option<Address>,
    pub meta: Option<serde_json::Value>,
}

/// Request to update a customer
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCustomerRequest {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 255))]
    pub first_name: Option<String>,
    #[validate(length(max = 255))]
    pub last_name: Option<String>,
    pub billing: Option<Address>,
    pub shipping: Option<Address>,
    pub meta: Option<serde_json::Value>,
}

/// Customer filter/query parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerFilter {
    pub email: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
    pub include: Option<Vec<Uuid>>,
    pub exclude: Option<Vec<Uuid>>,
    pub orderby: Option<CustomerOrderBy>,
    pub order: Option<super::product::SortOrder>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CustomerOrderBy {
    #[default]
    Registered,
    Id,
    Name,
    Email,
    OrdersCount,
    TotalSpent,
}
