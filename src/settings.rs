//! RustCommerce Settings

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Complete settings for RustCommerce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustCommerceSettings {
    pub general: GeneralSettings,
    pub products: ProductSettings,
    pub cart: CartSettings,
    pub checkout: CheckoutSettings,
    pub shipping: ShippingSettings,
    pub tax: TaxSettings,
    pub payments: PaymentSettings,
    pub emails: EmailSettings,
}

impl Default for RustCommerceSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            products: ProductSettings::default(),
            cart: CartSettings::default(),
            checkout: CheckoutSettings::default(),
            shipping: ShippingSettings::default(),
            tax: TaxSettings::default(),
            payments: PaymentSettings::default(),
            emails: EmailSettings::default(),
        }
    }
}

/// General store settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub store_name: String,
    pub store_address: String,
    pub store_address_2: String,
    pub store_city: String,
    pub store_state: String,
    pub store_postcode: String,
    pub store_country: String,
    pub currency: String,
    pub currency_position: CurrencyPosition,
    pub thousand_separator: String,
    pub decimal_separator: String,
    pub number_of_decimals: u8,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            store_name: "My Store".to_string(),
            store_address: String::new(),
            store_address_2: String::new(),
            store_city: String::new(),
            store_state: String::new(),
            store_postcode: String::new(),
            store_country: "US".to_string(),
            currency: "USD".to_string(),
            currency_position: CurrencyPosition::Left,
            thousand_separator: ",".to_string(),
            decimal_separator: ".".to_string(),
            number_of_decimals: 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyPosition {
    #[default]
    Left,
    Right,
    LeftSpace,
    RightSpace,
}

/// Product settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSettings {
    // Reviews
    pub enable_reviews: bool,
    pub review_rating_required: bool,
    pub verified_owners_only: bool,

    // Inventory
    pub enable_stock_management: bool,
    pub hold_stock_minutes: u32,
    pub low_stock_threshold: i32,
    pub out_of_stock_threshold: i32,
    pub out_of_stock_visibility: bool,
    pub enable_backorders: bool,

    // Display
    pub catalog_columns: u8,
    pub catalog_rows: u8,
    pub enable_ajax_add_to_cart: bool,
    pub redirect_after_add: bool,

    // Downloadable
    pub download_method: DownloadMethod,
    pub downloads_require_login: bool,
    pub grant_access_after_payment: bool,
}

impl Default for ProductSettings {
    fn default() -> Self {
        Self {
            enable_reviews: true,
            review_rating_required: true,
            verified_owners_only: false,
            enable_stock_management: true,
            hold_stock_minutes: 60,
            low_stock_threshold: 5,
            out_of_stock_threshold: 0,
            out_of_stock_visibility: true,
            enable_backorders: false,
            catalog_columns: 4,
            catalog_rows: 4,
            enable_ajax_add_to_cart: true,
            redirect_after_add: false,
            download_method: DownloadMethod::Force,
            downloads_require_login: false,
            grant_access_after_payment: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DownloadMethod {
    #[default]
    Force,
    XAccelRedirect,
    Redirect,
}

/// Cart settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartSettings {
    pub enable_cart: bool,
    pub enable_ajax_cart: bool,
    pub redirect_after_add: bool,
    pub enable_coupons: bool,
    pub enable_cart_cross_sells: bool,
    pub minimum_order_amount: Decimal,
    pub cart_page_id: Option<u64>,
}

impl Default for CartSettings {
    fn default() -> Self {
        Self {
            enable_cart: true,
            enable_ajax_cart: true,
            redirect_after_add: false,
            enable_coupons: true,
            enable_cart_cross_sells: true,
            minimum_order_amount: Decimal::ZERO,
            cart_page_id: None,
        }
    }
}

/// Checkout settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSettings {
    pub enable_guest_checkout: bool,
    pub force_ssl_checkout: bool,
    pub checkout_page_id: Option<u64>,
    pub terms_page_id: Option<u64>,
    pub privacy_page_id: Option<u64>,
    pub require_terms_acceptance: bool,
    pub enable_signup_and_login: bool,
    pub enable_order_notes: bool,
    pub require_phone: bool,
    pub require_company: bool,
}

impl Default for CheckoutSettings {
    fn default() -> Self {
        Self {
            enable_guest_checkout: true,
            force_ssl_checkout: true,
            checkout_page_id: None,
            terms_page_id: None,
            privacy_page_id: None,
            require_terms_acceptance: true,
            enable_signup_and_login: true,
            enable_order_notes: true,
            require_phone: false,
            require_company: false,
        }
    }
}

/// Shipping settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingSettings {
    pub enable_shipping: bool,
    pub enable_shipping_calculator: bool,
    pub hide_shipping_until_address: bool,
    pub shipping_destination: ShippingDestination,
    pub debug_mode: bool,
}

impl Default for ShippingSettings {
    fn default() -> Self {
        Self {
            enable_shipping: true,
            enable_shipping_calculator: true,
            hide_shipping_until_address: false,
            shipping_destination: ShippingDestination::ShippingAddress,
            debug_mode: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShippingDestination {
    #[default]
    ShippingAddress,
    BillingAddress,
    BillingOnly,
}

/// Tax settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSettings {
    pub enable_taxes: bool,
    pub prices_include_tax: bool,
    pub calculate_tax_based_on: TaxBasedOn,
    pub shipping_tax_class: String,
    pub rounding_at_subtotal: bool,
    pub additional_tax_classes: Vec<String>,
    pub display_prices_in_shop: TaxDisplay,
    pub display_prices_in_cart: TaxDisplay,
    pub price_display_suffix: String,
    pub display_tax_totals: TaxTotalsDisplay,
}

impl Default for TaxSettings {
    fn default() -> Self {
        Self {
            enable_taxes: true,
            prices_include_tax: false,
            calculate_tax_based_on: TaxBasedOn::ShippingAddress,
            shipping_tax_class: String::new(),
            rounding_at_subtotal: false,
            additional_tax_classes: vec!["reduced-rate".to_string(), "zero-rate".to_string()],
            display_prices_in_shop: TaxDisplay::ExcludingTax,
            display_prices_in_cart: TaxDisplay::ExcludingTax,
            price_display_suffix: String::new(),
            display_tax_totals: TaxTotalsDisplay::Itemized,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaxBasedOn {
    #[default]
    ShippingAddress,
    BillingAddress,
    ShopBaseAddress,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaxDisplay {
    IncludingTax,
    #[default]
    ExcludingTax,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaxTotalsDisplay {
    #[default]
    Itemized,
    Single,
}

/// Payment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSettings {
    pub default_gateway: String,
    pub enabled_gateways: Vec<String>,
}

impl Default for PaymentSettings {
    fn default() -> Self {
        Self {
            default_gateway: "stripe".to_string(),
            enabled_gateways: vec!["stripe".to_string(), "paypal".to_string()],
        }
    }
}

/// Email settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSettings {
    pub from_name: String,
    pub from_address: String,
    pub header_image: String,
    pub footer_text: String,
    pub base_color: String,
    pub background_color: String,
    pub body_background_color: String,
    pub text_color: String,
}

impl Default for EmailSettings {
    fn default() -> Self {
        Self {
            from_name: "My Store".to_string(),
            from_address: "noreply@example.com".to_string(),
            header_image: String::new(),
            footer_text: "Thank you for shopping with us!".to_string(),
            base_color: "#96588a".to_string(),
            background_color: "#f7f7f7".to_string(),
            body_background_color: "#ffffff".to_string(),
            text_color: "#3c3c3c".to_string(),
        }
    }
}
