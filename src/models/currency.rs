//! Multi-currency Support Models
//!
//! Self-contained currency management with exchange rates and conversion.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub code: String,       // ISO 4217 code (USD, EUR, GBP)
    pub name: String,
    pub symbol: String,
    pub symbol_position: SymbolPosition,
    pub decimal_separator: String,
    pub thousand_separator: String,
    pub decimals: i32,
    pub is_default: bool,
    pub is_active: bool,
    pub exchange_rate: Decimal, // Rate relative to base currency
    pub auto_update_rate: bool,
    pub rate_markup: Decimal,   // Additional markup on exchange rate
    pub rounding: RoundingMethod,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolPosition {
    Left,
    LeftSpace,
    Right,
    RightSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundingMethod {
    None,
    Up,
    Down,
    Nearest,
    NearestHalf,
    NearestQuarter,
}

/// Exchange rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub id: Uuid,
    pub from_currency: String,
    pub to_currency: String,
    pub rate: Decimal,
    pub source: RateSource,
    pub fetched_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RateSource {
    Manual,
    Api,
    Calculated,
}

/// Exchange rate history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateHistory {
    pub id: Uuid,
    pub from_currency: String,
    pub to_currency: String,
    pub rate: Decimal,
    pub recorded_at: DateTime<Utc>,
}

/// Currency-specific pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyPrice {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub currency_code: String,
    pub regular_price: Decimal,
    pub sale_price: Option<Decimal>,
    pub price_type: CurrencyPriceType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyPriceType {
    Manual,      // Manually set price
    Converted,   // Auto-converted from base currency
}

/// Currency zone (geo-based currency)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyZone {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub currency_code: String,
    pub countries: Vec<String>,
    pub auto_switch: bool,
    pub priority: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Price conversion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceConversion {
    pub original_amount: Decimal,
    pub original_currency: String,
    pub converted_amount: Decimal,
    pub target_currency: String,
    pub exchange_rate: Decimal,
    pub rate_with_markup: Decimal,
    pub formatted_original: String,
    pub formatted_converted: String,
}

/// Currency switcher widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencySwitcher {
    pub currencies: Vec<CurrencyOption>,
    pub current_currency: String,
    pub show_flags: bool,
    pub show_symbols: bool,
    pub show_names: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyOption {
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub flag_url: Option<String>,
}

/// Multi-currency settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencySettings {
    pub enabled: bool,
    pub base_currency: String,
    pub auto_detect_currency: bool,
    pub detection_method: CurrencyDetection,
    pub show_currency_switcher: bool,
    pub switcher_position: SwitcherPosition,
    pub convert_prices_in_cart: bool,
    pub show_both_prices: bool,
    pub exchange_rate_api: Option<String>,
    pub api_key: Option<String>,
    pub rate_update_frequency_hours: i32,
    pub round_converted_prices: bool,
    pub rounding_increment: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyDetection {
    Geolocation,
    BrowserLanguage,
    UserPreference,
    ShippingCountry,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitcherPosition {
    Header,
    Footer,
    Sidebar,
    Custom,
}

impl Default for CurrencySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            base_currency: "USD".to_string(),
            auto_detect_currency: true,
            detection_method: CurrencyDetection::Geolocation,
            show_currency_switcher: true,
            switcher_position: SwitcherPosition::Header,
            convert_prices_in_cart: true,
            show_both_prices: false,
            exchange_rate_api: None,
            api_key: None,
            rate_update_frequency_hours: 24,
            round_converted_prices: true,
            rounding_increment: Some(Decimal::new(1, 2)), // 0.01
        }
    }
}

impl Currency {
    /// Format amount in this currency
    pub fn format_amount(&self, amount: Decimal) -> String {
        let rounded = self.round_amount(amount);
        let formatted = format!("{:.prec$}", rounded, prec = self.decimals as usize);

        // Apply separators
        let parts: Vec<&str> = formatted.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1).unwrap_or(&"");

        // Add thousand separators
        let integer_with_sep = self.add_thousand_separators(integer_part);

        let number = if self.decimals > 0 && !decimal_part.is_empty() {
            format!("{}{}{}", integer_with_sep, self.decimal_separator, decimal_part)
        } else {
            integer_with_sep
        };

        // Position symbol
        match self.symbol_position {
            SymbolPosition::Left => format!("{}{}", self.symbol, number),
            SymbolPosition::LeftSpace => format!("{} {}", self.symbol, number),
            SymbolPosition::Right => format!("{}{}", number, self.symbol),
            SymbolPosition::RightSpace => format!("{} {}", number, self.symbol),
        }
    }

    fn add_thousand_separators(&self, s: &str) -> String {
        let chars: Vec<char> = s.chars().rev().collect();
        let mut result = String::new();
        for (i, c) in chars.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push_str(&self.thousand_separator);
            }
            result.push(*c);
        }
        result.chars().rev().collect()
    }

    /// Round amount according to currency settings
    pub fn round_amount(&self, amount: Decimal) -> Decimal {
        let factor = Decimal::from(10_i64.pow(self.decimals as u32));
        match self.rounding {
            RoundingMethod::None => amount,
            RoundingMethod::Up => (amount * factor).ceil() / factor,
            RoundingMethod::Down => (amount * factor).floor() / factor,
            RoundingMethod::Nearest => (amount * factor).round() / factor,
            RoundingMethod::NearestHalf => {
                let doubled = amount * Decimal::from(2) * factor;
                doubled.round() / (Decimal::from(2) * factor)
            }
            RoundingMethod::NearestQuarter => {
                let quadrupled = amount * Decimal::from(4) * factor;
                quadrupled.round() / (Decimal::from(4) * factor)
            }
        }
    }

    /// Convert amount to another currency
    pub fn convert_to(&self, amount: Decimal, target: &Currency) -> Decimal {
        // Convert to base currency first, then to target
        let base_amount = amount / self.exchange_rate;
        let target_amount = base_amount * target.exchange_rate * (Decimal::ONE + target.rate_markup / Decimal::from(100));
        target.round_amount(target_amount)
    }
}

/// Convert price request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertPriceRequest {
    pub amount: Decimal,
    pub from_currency: String,
    pub to_currency: String,
}

/// Get currencies response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrenciesResponse {
    pub currencies: Vec<Currency>,
    pub default_currency: String,
    pub current_currency: String,
}

/// Update exchange rates request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExchangeRatesRequest {
    pub rates: HashMap<String, Decimal>,
    pub source: RateSource,
}
