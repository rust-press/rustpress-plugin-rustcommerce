//! Payment Models

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Payment gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGatewayConfig {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub gateway_id: String,
    pub title: String,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub method_order: i32,
    pub settings: serde_json::Value,
    pub supports: Vec<String>,
    pub countries: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Saved payment token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentToken {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub gateway_id: String,
    pub token: String,
    pub token_type: PaymentTokenType,
    pub last_four: Option<String>,
    pub expiry_month: Option<String>,
    pub expiry_year: Option<String>,
    pub card_type: Option<CardType>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentTokenType {
    CreditCard,
    ECheck,
    ApplePay,
    GooglePay,
    PayPal,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardType {
    Visa,
    Mastercard,
    Amex,
    Discover,
    Diners,
    Jcb,
    UnionPay,
    Other,
}

impl CardType {
    pub fn from_number(number: &str) -> Option<Self> {
        let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();

        if digits.starts_with('4') {
            Some(Self::Visa)
        } else if digits.starts_with("51") || digits.starts_with("52") || digits.starts_with("53") || digits.starts_with("54") || digits.starts_with("55") {
            Some(Self::Mastercard)
        } else if digits.starts_with("34") || digits.starts_with("37") {
            Some(Self::Amex)
        } else if digits.starts_with("6011") || digits.starts_with("65") {
            Some(Self::Discover)
        } else if digits.starts_with("36") || digits.starts_with("38") {
            Some(Self::Diners)
        } else if digits.starts_with("35") {
            Some(Self::Jcb)
        } else if digits.starts_with("62") {
            Some(Self::UnionPay)
        } else {
            None
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Visa => "Visa",
            Self::Mastercard => "Mastercard",
            Self::Amex => "American Express",
            Self::Discover => "Discover",
            Self::Diners => "Diners Club",
            Self::Jcb => "JCB",
            Self::UnionPay => "UnionPay",
            Self::Other => "Card",
        }
    }
}

/// Payment transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub order_id: Uuid,
    pub transaction_id: String,
    pub gateway_id: String,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub status: TransactionStatus,
    pub gateway_response: serde_json::Value,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Payment,
    Refund,
    Void,
    Capture,
    Authorization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
    Cancelled,
}

/// Payment request for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub order_id: Uuid,
    pub amount: Decimal,
    pub currency: String,

    // Payment method
    pub gateway_id: String,
    pub payment_token: Option<Uuid>,
    pub card: Option<CardDetails>,
    pub save_payment_method: bool,

    // Customer info
    pub customer_id: Option<Uuid>,
    pub billing_email: String,
    pub billing_name: String,
    pub billing_address: Option<BillingAddress>,

    // Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Card details for new payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub number: String,
    pub exp_month: String,
    pub exp_year: String,
    pub cvc: String,
    pub name: Option<String>,
}

/// Billing address for payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

/// Payment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResult {
    pub success: bool,
    pub transaction_id: Option<String>,
    pub status: TransactionStatus,
    pub message: Option<String>,
    pub redirect_url: Option<String>,
    pub requires_action: bool,
    pub action_url: Option<String>,
    pub raw_response: Option<serde_json::Value>,
}

impl PaymentResult {
    pub fn success(transaction_id: String) -> Self {
        Self {
            success: true,
            transaction_id: Some(transaction_id),
            status: TransactionStatus::Completed,
            message: None,
            redirect_url: None,
            requires_action: false,
            action_url: None,
            raw_response: None,
        }
    }

    pub fn pending(transaction_id: String, message: Option<String>) -> Self {
        Self {
            success: true,
            transaction_id: Some(transaction_id),
            status: TransactionStatus::Pending,
            message,
            redirect_url: None,
            requires_action: false,
            action_url: None,
            raw_response: None,
        }
    }

    pub fn requires_action(action_url: String) -> Self {
        Self {
            success: true,
            transaction_id: None,
            status: TransactionStatus::Pending,
            message: Some("Additional action required".to_string()),
            redirect_url: None,
            requires_action: true,
            action_url: Some(action_url),
            raw_response: None,
        }
    }

    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            transaction_id: None,
            status: TransactionStatus::Failed,
            message: Some(error),
            redirect_url: None,
            requires_action: false,
            action_url: None,
            raw_response: None,
        }
    }
}

/// Refund request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub transaction_id: String,
    pub amount: Option<Decimal>, // None = full refund
    pub reason: Option<String>,
}

/// Refund result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResult {
    pub success: bool,
    pub refund_id: Option<String>,
    pub amount: Decimal,
    pub message: Option<String>,
    pub raw_response: Option<serde_json::Value>,
}

/// Gateway feature support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GatewayFeature {
    Products,
    Subscriptions,
    Refunds,
    Tokenization,
    SavePaymentMethod,
    AddPaymentMethod,
    ApplePay,
    GooglePay,
    ThreeDSecure,
    PreOrders,
    MultiCurrency,
}

/// Store credit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreCredit {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Store credit transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreCreditTransaction {
    pub id: Uuid,
    pub store_credit_id: Uuid,
    pub amount: Decimal,
    pub transaction_type: StoreCreditTransactionType,
    pub reason: Option<String>,
    pub order_id: Option<Uuid>,
    pub balance_after: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoreCreditTransactionType {
    Credit,
    Debit,
    Refund,
    Adjustment,
}

/// Gift card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCard {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub code: String,
    pub initial_balance: Decimal,
    pub current_balance: Decimal,
    pub purchaser_email: Option<String>,
    pub purchaser_name: Option<String>,
    pub recipient_email: Option<String>,
    pub recipient_name: Option<String>,
    pub personal_message: Option<String>,
    pub send_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub order_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl GiftCard {
    /// Check if gift card is valid
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if self.current_balance <= Decimal::ZERO {
            return false;
        }

        if let Some(expires) = self.expires_at {
            if Utc::now() > expires {
                return false;
            }
        }

        true
    }
}
