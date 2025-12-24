//! Tax Service
//!
//! Handles tax calculations, tax rates, and tax display.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::tax::{TaxRate, TaxClass, TaxLocation, CalculatedTax, TaxCalculationResult};
use crate::models::cart::Cart;
use crate::settings::RustCommerceSettings;

/// Tax service
pub struct TaxService {
    settings: RustCommerceSettings,
}

/// Tax calculation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxCalculationMode {
    Inclusive,  // Prices include tax
    Exclusive,  // Prices exclude tax (tax added)
}

/// Tax display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxDisplayMode {
    Including,  // Display with tax included
    Excluding,  // Display without tax
}

impl TaxService {
    /// Create a new tax service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Check if taxes are enabled
    pub fn taxes_enabled(&self) -> bool {
        self.settings.tax.enable_taxes
    }

    /// Get applicable tax rates for a location
    pub fn get_rates_for_location(
        &self,
        location: &TaxLocation,
        tax_class: &str,
        rates: &[TaxRate],
    ) -> Vec<&TaxRate> {
        let mut applicable: Vec<&TaxRate> = rates.iter()
            .filter(|r| {
                r.tax_class == tax_class &&
                r.matches_location(&location.country, &location.state, &location.postcode, &location.city)
            })
            .collect();

        // Sort by priority
        applicable.sort_by(|a, b| a.priority.cmp(&b.priority));

        applicable
    }

    /// Calculate tax for a single amount
    pub fn calculate_tax(
        &self,
        amount: Decimal,
        location: &TaxLocation,
        tax_class: &str,
        rates: &[TaxRate],
        prices_include_tax: bool,
    ) -> TaxCalculationResult {
        if !self.taxes_enabled() {
            return TaxCalculationResult {
                taxes: vec![],
                total_tax: Decimal::ZERO,
                total_shipping_tax: Decimal::ZERO,
            };
        }

        let applicable_rates = self.get_rates_for_location(location, tax_class, rates);

        if applicable_rates.is_empty() {
            return TaxCalculationResult {
                taxes: vec![],
                total_tax: Decimal::ZERO,
                total_shipping_tax: Decimal::ZERO,
            };
        }

        let mut taxes = Vec::new();
        let mut total_tax = Decimal::ZERO;
        let mut running_subtotal = amount;

        // Group by priority for compound calculation
        let mut priority_groups: HashMap<i32, Vec<&TaxRate>> = HashMap::new();
        for rate in &applicable_rates {
            priority_groups.entry(rate.priority)
                .or_insert_with(Vec::new)
                .push(rate);
        }

        let mut priorities: Vec<i32> = priority_groups.keys().cloned().collect();
        priorities.sort();

        for priority in priorities {
            let group_rates = &priority_groups[&priority];
            let mut group_tax = Decimal::ZERO;

            for rate in group_rates {
                let base_amount = if rate.compound {
                    running_subtotal + total_tax
                } else {
                    running_subtotal
                };

                let tax_amount = if prices_include_tax {
                    self.calculate_tax_from_inclusive(base_amount, rate.rate)
                } else {
                    self.calculate_tax_from_exclusive(base_amount, rate.rate)
                };

                taxes.push(CalculatedTax {
                    rate_id: rate.id,
                    rate_code: format!("{}_{}", rate.country, rate.name),
                    label: rate.name.clone(),
                    rate: rate.rate,
                    compound: rate.compound,
                    tax_amount,
                    shipping_tax_amount: Decimal::ZERO,
                });

                group_tax += tax_amount;
            }

            total_tax += group_tax;
        }

        TaxCalculationResult {
            taxes,
            total_tax,
            total_shipping_tax: Decimal::ZERO,
        }
    }

    /// Calculate tax from tax-exclusive price
    fn calculate_tax_from_exclusive(&self, amount: Decimal, rate: Decimal) -> Decimal {
        (amount * rate / dec!(100)).round_dp(self.settings.general.number_of_decimals as u32)
    }

    /// Calculate tax from tax-inclusive price
    fn calculate_tax_from_inclusive(&self, amount: Decimal, rate: Decimal) -> Decimal {
        let divisor = dec!(1) + (rate / dec!(100));
        let pre_tax = amount / divisor;
        (amount - pre_tax).round_dp(self.settings.general.number_of_decimals as u32)
    }

    /// Calculate shipping tax
    pub fn calculate_shipping_tax(
        &self,
        shipping_cost: Decimal,
        location: &TaxLocation,
        rates: &[TaxRate],
    ) -> Decimal {
        if !self.taxes_enabled() {
            return Decimal::ZERO;
        }

        // Get rates that apply to shipping
        let shipping_rates: Vec<&TaxRate> = rates.iter()
            .filter(|r| {
                r.shipping &&
                r.matches_location(&location.country, &location.state, &location.postcode, &location.city)
            })
            .collect();

        let mut total_tax = Decimal::ZERO;
        for rate in shipping_rates {
            total_tax += self.calculate_tax_from_exclusive(shipping_cost, rate.rate);
        }

        total_tax
    }

    /// Get price including tax
    pub fn get_price_including_tax(&self, price: Decimal, tax_rate: Decimal) -> Decimal {
        price + (price * tax_rate / dec!(100))
    }

    /// Get price excluding tax
    pub fn get_price_excluding_tax(&self, price_with_tax: Decimal, tax_rate: Decimal) -> Decimal {
        price_with_tax / (dec!(1) + tax_rate / dec!(100))
    }

    /// Format tax rate for display
    pub fn format_tax_rate(&self, rate: Decimal) -> String {
        format!("{}%", rate.round_dp(2))
    }

    /// Get tax label for display
    pub fn get_tax_label(&self, taxes: &[CalculatedTax]) -> String {
        if taxes.len() == 1 {
            taxes[0].label.clone()
        } else if taxes.is_empty() {
            "Tax".to_string()
        } else {
            "Taxes".to_string()
        }
    }

    /// Calculate cart taxes
    pub fn calculate_cart_taxes(
        &self,
        cart: &Cart,
        location: &TaxLocation,
        rates: &[TaxRate],
        product_tax_classes: &HashMap<Uuid, String>,
    ) -> CartTaxResult {
        if !self.taxes_enabled() {
            return CartTaxResult::default();
        }

        let prices_include_tax = self.settings.tax.prices_include_tax;
        let mut item_taxes = HashMap::new();
        let mut total_item_tax = Decimal::ZERO;

        // Calculate tax for each item
        for item in &cart.items {
            let tax_class = product_tax_classes.get(&item.product_id)
                .cloned()
                .unwrap_or_else(|| "standard".to_string());

            let result = self.calculate_tax(
                item.line_subtotal,
                location,
                &tax_class,
                rates,
                prices_include_tax,
            );

            item_taxes.insert(item.key.clone(), result.total_tax);
            total_item_tax += result.total_tax;
        }

        // Calculate shipping tax
        let shipping_tax = self.calculate_shipping_tax(
            cart.totals.shipping_total,
            location,
            rates,
        );

        CartTaxResult {
            item_taxes,
            total_item_tax,
            shipping_tax,
            total_tax: total_item_tax + shipping_tax,
        }
    }

    /// Get default tax class
    pub fn get_default_tax_class(&self) -> &str {
        "standard"
    }

    /// Get tax classes
    pub fn get_tax_classes(&self) -> Vec<TaxClassInfo> {
        vec![
            TaxClassInfo {
                slug: "standard".to_string(),
                name: "Standard rate".to_string(),
            },
            TaxClassInfo {
                slug: "reduced-rate".to_string(),
                name: "Reduced rate".to_string(),
            },
            TaxClassInfo {
                slug: "zero-rate".to_string(),
                name: "Zero rate".to_string(),
            },
        ]
    }

    /// Check if tax is applied based on shipping or billing address
    pub fn tax_based_on(&self) -> TaxBasedOn {
        // Could be configurable
        TaxBasedOn::ShippingAddress
    }
}

/// Cart tax calculation result
#[derive(Debug, Clone, Default)]
pub struct CartTaxResult {
    pub item_taxes: HashMap<String, Decimal>,
    pub total_item_tax: Decimal,
    pub shipping_tax: Decimal,
    pub total_tax: Decimal,
}

/// Tax class info
#[derive(Debug, Clone)]
pub struct TaxClassInfo {
    pub slug: String,
    pub name: String,
}

/// Tax based on setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxBasedOn {
    ShippingAddress,
    BillingAddress,
    ShopBaseAddress,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_rate(rate: Decimal, country: &str, priority: i32, compound: bool) -> TaxRate {
        TaxRate {
            id: Uuid::new_v4(),
            site_id: None,
            country: country.to_string(),
            state: String::new(),
            postcode: String::new(),
            city: String::new(),
            rate,
            name: "Test Tax".to_string(),
            priority,
            compound,
            shipping: true,
            tax_order: 0,
            tax_class: "standard".to_string(),
            created_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_calculate_tax_exclusive() {
        let settings = RustCommerceSettings::default();
        let service = TaxService::new(settings);

        let location = TaxLocation::new("US", "CA", "90210", "Beverly Hills");
        let rates = vec![create_test_rate(dec!(10), "US", 1, false)];

        let result = service.calculate_tax(
            dec!(100),
            &location,
            "standard",
            &rates,
            false,
        );

        assert_eq!(result.total_tax, dec!(10));
    }

    #[test]
    fn test_calculate_tax_inclusive() {
        let settings = RustCommerceSettings::default();
        let service = TaxService::new(settings);

        let location = TaxLocation::new("US", "CA", "90210", "Beverly Hills");
        let rates = vec![create_test_rate(dec!(10), "US", 1, false)];

        let result = service.calculate_tax(
            dec!(110),
            &location,
            "standard",
            &rates,
            true,
        );

        // 110 / 1.1 = 100, tax = 10
        assert_eq!(result.total_tax, dec!(10));
    }
}
