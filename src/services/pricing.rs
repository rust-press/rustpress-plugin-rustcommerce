//! Pricing Service
//!
//! Handles price calculations, sale prices, and price display formatting.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::settings::{RustCommerceSettings, CurrencyPosition};
use crate::models::product::{Product, ProductVariation};

/// Pricing service
pub struct PricingService {
    settings: RustCommerceSettings,
}

impl PricingService {
    /// Create a new pricing service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Get the effective price for a product
    pub fn get_price(&self, product: &Product) -> Option<Decimal> {
        product.get_price()
    }

    /// Get the effective price for a variation
    pub fn get_variation_price(&self, product: &Product, variation: &ProductVariation) -> Option<Decimal> {
        // Check if variation has its own price
        if variation.sale_price.is_some() || variation.regular_price.is_some() {
            let now = chrono::Utc::now();

            // Check sale price date range
            let sale_active = if variation.sale_price.is_some() {
                let from_ok = variation.sale_price_from.map_or(true, |d| now >= d);
                let to_ok = variation.sale_price_to.map_or(true, |d| now <= d);
                from_ok && to_ok
            } else {
                false
            };

            if sale_active {
                variation.sale_price
            } else {
                variation.regular_price
            }
        } else {
            // Fall back to product price
            product.get_price()
        }
    }

    /// Format a price for display
    pub fn format_price(&self, price: Decimal) -> String {
        let formatted = self.format_decimal(price);
        let symbol = self.get_currency_symbol();

        match self.settings.general.currency_position {
            CurrencyPosition::Left => format!("{}{}", symbol, formatted),
            CurrencyPosition::Right => format!("{}{}", formatted, symbol),
            CurrencyPosition::LeftSpace => format!("{} {}", symbol, formatted),
            CurrencyPosition::RightSpace => format!("{} {}", formatted, symbol),
        }
    }

    /// Format a decimal number with proper separators
    pub fn format_decimal(&self, value: Decimal) -> String {
        let decimals = self.settings.general.number_of_decimals as u32;
        let rounded = value.round_dp(decimals);

        let (integer, decimal) = {
            let str_val = format!("{:.prec$}", rounded, prec = decimals as usize);
            if let Some(pos) = str_val.find('.') {
                (str_val[..pos].to_string(), str_val[pos + 1..].to_string())
            } else {
                (str_val, "0".repeat(decimals as usize))
            }
        };

        // Add thousand separators
        let integer_with_sep = self.add_thousand_separators(&integer);

        if decimals > 0 {
            format!(
                "{}{}{}",
                integer_with_sep,
                self.settings.general.decimal_separator,
                decimal
            )
        } else {
            integer_with_sep
        }
    }

    /// Add thousand separators to integer part
    fn add_thousand_separators(&self, integer: &str) -> String {
        let chars: Vec<char> = integer.chars().rev().collect();
        let mut result = String::new();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push_str(&self.settings.general.thousand_separator);
            }
            result.push(*c);
        }

        result.chars().rev().collect()
    }

    /// Get currency symbol
    pub fn get_currency_symbol(&self) -> &str {
        match self.settings.general.currency.as_str() {
            "USD" => "$",
            "EUR" => "€",
            "GBP" => "£",
            "JPY" => "¥",
            "CNY" => "¥",
            "INR" => "₹",
            "AUD" => "A$",
            "CAD" => "C$",
            "CHF" => "CHF",
            "KRW" => "₩",
            "RUB" => "₽",
            "BRL" => "R$",
            "MXN" => "Mex$",
            "PLN" => "zł",
            "SEK" => "kr",
            "NOK" => "kr",
            "DKK" => "kr",
            "HKD" => "HK$",
            "SGD" => "S$",
            "NZD" => "NZ$",
            "ZAR" => "R",
            "THB" => "฿",
            "PHP" => "₱",
            "IDR" => "Rp",
            "MYR" => "RM",
            "AED" => "د.إ",
            "SAR" => "﷼",
            "TRY" => "₺",
            "ILS" => "₪",
            "CZK" => "Kč",
            "HUF" => "Ft",
            "RON" => "lei",
            "TWD" => "NT$",
            "VND" => "₫",
            "CLP" => "CLP$",
            "COP" => "COP$",
            "ARS" => "ARS$",
            "PEN" => "S/",
            "EGP" => "E£",
            "NGN" => "₦",
            "PKR" => "₨",
            "BDT" => "৳",
            _ => "$", // Default to USD
        }
    }

    /// Calculate sale percentage
    pub fn calculate_sale_percentage(&self, regular_price: Decimal, sale_price: Decimal) -> Decimal {
        if regular_price <= Decimal::ZERO {
            return Decimal::ZERO;
        }

        let discount = regular_price - sale_price;
        (discount / regular_price * dec!(100)).round_dp(0)
    }

    /// Calculate price with tax
    pub fn get_price_including_tax(&self, price: Decimal, tax_rate: Decimal) -> Decimal {
        price + (price * tax_rate / dec!(100))
    }

    /// Calculate price without tax
    pub fn get_price_excluding_tax(&self, price_with_tax: Decimal, tax_rate: Decimal) -> Decimal {
        price_with_tax / (dec!(1) + tax_rate / dec!(100))
    }

    /// Get price range for a variable product
    pub fn get_price_range(&self, variations: &[ProductVariation]) -> Option<(Decimal, Decimal)> {
        let prices: Vec<Decimal> = variations
            .iter()
            .filter_map(|v| v.regular_price.or(v.sale_price))
            .collect();

        if prices.is_empty() {
            return None;
        }

        let min = prices.iter().min().copied()?;
        let max = prices.iter().max().copied()?;

        Some((min, max))
    }

    /// Format price range for display
    pub fn format_price_range(&self, min: Decimal, max: Decimal) -> String {
        if min == max {
            self.format_price(min)
        } else {
            format!("{} – {}", self.format_price(min), self.format_price(max))
        }
    }

    /// Calculate line item total
    pub fn calculate_line_total(&self, unit_price: Decimal, quantity: i32) -> Decimal {
        unit_price * Decimal::from(quantity)
    }

    /// Apply bulk/tiered pricing
    pub fn apply_tiered_pricing(
        &self,
        base_price: Decimal,
        quantity: i32,
        tiers: &[(i32, Decimal)], // (min_quantity, price)
    ) -> Decimal {
        // Find the applicable tier
        let mut applicable_price = base_price;

        for (min_qty, tier_price) in tiers.iter().rev() {
            if quantity >= *min_qty {
                applicable_price = *tier_price;
                break;
            }
        }

        applicable_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_price() {
        let settings = RustCommerceSettings::default();
        let service = PricingService::new(settings);

        assert_eq!(service.format_price(dec!(19.99)), "$19.99");
        assert_eq!(service.format_price(dec!(1234.56)), "$1,234.56");
        assert_eq!(service.format_price(dec!(0)), "$0.00");
    }

    #[test]
    fn test_sale_percentage() {
        let settings = RustCommerceSettings::default();
        let service = PricingService::new(settings);

        assert_eq!(
            service.calculate_sale_percentage(dec!(100), dec!(75)),
            dec!(25)
        );
        assert_eq!(
            service.calculate_sale_percentage(dec!(50), dec!(25)),
            dec!(50)
        );
    }

    #[test]
    fn test_price_with_tax() {
        let settings = RustCommerceSettings::default();
        let service = PricingService::new(settings);

        let price_with_tax = service.get_price_including_tax(dec!(100), dec!(10));
        assert_eq!(price_with_tax, dec!(110));

        let price_without_tax = service.get_price_excluding_tax(dec!(110), dec!(10));
        assert_eq!(price_without_tax, dec!(100));
    }
}
