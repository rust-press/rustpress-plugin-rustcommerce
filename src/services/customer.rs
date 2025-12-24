//! Customer Service
//!
//! Handles customer management, authentication, and customer data.

use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::models::customer::{Customer, CustomerAddress};
use crate::models::order::Order;
use crate::settings::RustCommerceSettings;

/// Customer service
pub struct CustomerService {
    settings: RustCommerceSettings,
}

/// Customer filter
#[derive(Debug, Clone, Default)]
pub struct CustomerFilter {
    pub search: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub date_registered_from: Option<DateTime<Utc>>,
    pub date_registered_to: Option<DateTime<Utc>>,
    pub total_spent_min: Option<Decimal>,
    pub total_spent_max: Option<Decimal>,
    pub order_count_min: Option<i32>,
    pub order_count_max: Option<i32>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub order_by: Option<String>,
    pub order: Option<String>,
}

/// Customer statistics
#[derive(Debug, Clone)]
pub struct CustomerStats {
    pub customer_id: Uuid,
    pub total_orders: i32,
    pub total_spent: Decimal,
    pub average_order_value: Decimal,
    pub first_order_date: Option<DateTime<Utc>>,
    pub last_order_date: Option<DateTime<Utc>>,
    pub orders_by_status: HashMap<String, i32>,
}

/// Customer error
#[derive(Debug, Clone)]
pub enum CustomerError {
    NotFound,
    EmailAlreadyExists,
    InvalidEmail,
    InvalidPassword,
    AccountDisabled,
}

impl std::fmt::Display for CustomerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Customer not found"),
            Self::EmailAlreadyExists => write!(f, "Email already exists"),
            Self::InvalidEmail => write!(f, "Invalid email address"),
            Self::InvalidPassword => write!(f, "Invalid password"),
            Self::AccountDisabled => write!(f, "Account is disabled"),
        }
    }
}

impl CustomerService {
    /// Create a new customer service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Create a new customer
    pub fn create_customer(
        &self,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        user_id: Option<Uuid>,
    ) -> Result<Customer, CustomerError> {
        if !self.is_valid_email(&email) {
            return Err(CustomerError::InvalidEmail);
        }

        Ok(Customer {
            id: Uuid::now_v7(),
            site_id: None,
            user_id,
            email,
            first_name: first_name.unwrap_or_default(),
            last_name: last_name.unwrap_or_default(),
            display_name: None,
            role: "customer".to_string(),
            billing_address: None,
            shipping_address: None,
            is_paying_customer: false,
            avatar_url: None,
            meta: HashMap::new(),
            created_at: Utc::now(),
            updated_at: None,
            last_active: Some(Utc::now()),
        })
    }

    /// Update customer
    pub fn update_customer(
        &self,
        customer: &mut Customer,
        first_name: Option<String>,
        last_name: Option<String>,
        billing_address: Option<CustomerAddress>,
        shipping_address: Option<CustomerAddress>,
    ) {
        if let Some(name) = first_name {
            customer.first_name = name;
        }
        if let Some(name) = last_name {
            customer.last_name = name;
        }
        if billing_address.is_some() {
            customer.billing_address = billing_address;
        }
        if shipping_address.is_some() {
            customer.shipping_address = shipping_address;
        }
        customer.updated_at = Some(Utc::now());
    }

    /// Validate email format
    fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() >= 5
    }

    /// Calculate customer statistics
    pub fn calculate_stats(&self, customer: &Customer, orders: &[Order]) -> CustomerStats {
        let customer_orders: Vec<&Order> = orders.iter()
            .filter(|o| o.customer_id == Some(customer.id))
            .collect();

        let total_orders = customer_orders.len() as i32;
        let total_spent: Decimal = customer_orders.iter()
            .map(|o| o.total)
            .sum();

        let average_order_value = if total_orders > 0 {
            total_spent / Decimal::from(total_orders)
        } else {
            Decimal::ZERO
        };

        let first_order_date = customer_orders.iter()
            .map(|o| o.created_at)
            .min();

        let last_order_date = customer_orders.iter()
            .map(|o| o.created_at)
            .max();

        let mut orders_by_status: HashMap<String, i32> = HashMap::new();
        for order in &customer_orders {
            let status = format!("{:?}", order.status).to_lowercase();
            *orders_by_status.entry(status).or_insert(0) += 1;
        }

        CustomerStats {
            customer_id: customer.id,
            total_orders,
            total_spent,
            average_order_value,
            first_order_date,
            last_order_date,
            orders_by_status,
        }
    }

    /// Get customer display name
    pub fn get_display_name(&self, customer: &Customer) -> String {
        if let Some(ref display_name) = customer.display_name {
            if !display_name.is_empty() {
                return display_name.clone();
            }
        }

        let full_name = format!("{} {}", customer.first_name, customer.last_name).trim().to_string();
        if !full_name.is_empty() {
            full_name
        } else {
            customer.email.split('@').next().unwrap_or("Customer").to_string()
        }
    }

    /// Check if customer is paying customer
    pub fn is_paying_customer(&self, orders: &[Order]) -> bool {
        orders.iter().any(|o| o.paid_at.is_some())
    }

    /// Get customer's saved addresses
    pub fn get_addresses(&self, customer: &Customer) -> Vec<CustomerAddress> {
        let mut addresses = Vec::new();

        if let Some(ref billing) = customer.billing_address {
            addresses.push(billing.clone());
        }

        if let Some(ref shipping) = customer.shipping_address {
            // Only add if different from billing
            if customer.billing_address.as_ref() != Some(shipping) {
                addresses.push(shipping.clone());
            }
        }

        addresses
    }

    /// Update billing address
    pub fn update_billing_address(&self, customer: &mut Customer, address: CustomerAddress) {
        customer.billing_address = Some(address);
        customer.updated_at = Some(Utc::now());
    }

    /// Update shipping address
    pub fn update_shipping_address(&self, customer: &mut Customer, address: CustomerAddress) {
        customer.shipping_address = Some(address);
        customer.updated_at = Some(Utc::now());
    }

    /// Format customer for list display
    pub fn format_customer_summary(&self, customer: &Customer, orders: &[Order]) -> CustomerSummary {
        let stats = self.calculate_stats(customer, orders);

        CustomerSummary {
            id: customer.id,
            email: customer.email.clone(),
            display_name: self.get_display_name(customer),
            avatar_url: customer.avatar_url.clone(),
            total_orders: stats.total_orders,
            total_spent: stats.total_spent,
            last_order_date: stats.last_order_date,
            created_at: customer.created_at,
        }
    }

    /// Generate gravatar URL for customer
    pub fn get_gravatar_url(&self, email: &str, size: u32) -> String {
        let hash = format!("{:x}", md5::compute(email.trim().to_lowercase()));
        format!(
            "https://www.gravatar.com/avatar/{}?s={}&d=mp",
            hash, size
        )
    }

    /// Check if customer can download a product
    pub fn can_download(&self, _customer_id: Uuid, _download_id: Uuid) -> bool {
        // In full implementation, check download permissions table
        true
    }

    /// Get customer's recent orders
    pub fn get_recent_orders<'a>(&self, customer: &Customer, orders: &'a [Order], limit: usize) -> Vec<&'a Order> {
        let mut customer_orders: Vec<&Order> = orders.iter()
            .filter(|o| o.customer_id == Some(customer.id))
            .collect();

        customer_orders.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        customer_orders.truncate(limit);
        customer_orders
    }
}

/// Customer summary for lists
#[derive(Debug, Clone)]
pub struct CustomerSummary {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub total_orders: i32,
    pub total_spent: Decimal,
    pub last_order_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Customer download permission
#[derive(Debug, Clone)]
pub struct DownloadPermission {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub download_id: Uuid,
    pub download_count: i32,
    pub downloads_remaining: Option<i32>,
    pub access_expires: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_customer() {
        let settings = RustCommerceSettings::default();
        let service = CustomerService::new(settings);

        let customer = service.create_customer(
            "test@example.com".to_string(),
            Some("John".to_string()),
            Some("Doe".to_string()),
            None,
        ).unwrap();

        assert_eq!(customer.email, "test@example.com");
        assert_eq!(customer.first_name, "John");
        assert_eq!(customer.last_name, "Doe");
    }

    #[test]
    fn test_display_name() {
        let settings = RustCommerceSettings::default();
        let service = CustomerService::new(settings);

        let mut customer = service.create_customer(
            "test@example.com".to_string(),
            Some("John".to_string()),
            Some("Doe".to_string()),
            None,
        ).unwrap();

        assert_eq!(service.get_display_name(&customer), "John Doe");

        customer.first_name = String::new();
        customer.last_name = String::new();
        assert_eq!(service.get_display_name(&customer), "test");
    }

    #[test]
    fn test_invalid_email() {
        let settings = RustCommerceSettings::default();
        let service = CustomerService::new(settings);

        let result = service.create_customer(
            "invalid".to_string(),
            None,
            None,
            None,
        );

        assert!(matches!(result, Err(CustomerError::InvalidEmail)));
    }
}
