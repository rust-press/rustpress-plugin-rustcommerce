//! BACS (Direct Bank Transfer) Payment Gateway
//!
//! Payment method for direct bank transfers.

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::gateway::{
    PaymentGateway, GatewayError, GatewaySettingField, SettingFieldType,
    TokenizeRequest, WebhookResult,
};
use crate::models::payment::{
    PaymentRequest, PaymentResult, RefundRequest, RefundResult,
    PaymentToken, GatewayFeature, TransactionStatus,
};

/// BACS gateway configuration
#[derive(Debug, Clone)]
pub struct BacsConfig {
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub account_name: String,
    pub account_number: String,
    pub sort_code: String,
    pub bank_name: String,
    pub iban: String,
    pub bic: String,
}

impl Default for BacsConfig {
    fn default() -> Self {
        Self {
            title: "Direct bank transfer".to_string(),
            description: "Make your payment directly into our bank account. Please use your Order ID as the payment reference. Your order will not be shipped until the funds have cleared in our account.".to_string(),
            instructions: String::new(),
            account_name: String::new(),
            account_number: String::new(),
            sort_code: String::new(),
            bank_name: String::new(),
            iban: String::new(),
            bic: String::new(),
        }
    }
}

/// BACS (Direct Bank Transfer) payment gateway
pub struct BacsGateway {
    config: BacsConfig,
    enabled: bool,
}

impl BacsGateway {
    /// Create a new BACS gateway
    pub fn new(config: BacsConfig) -> Self {
        Self {
            config,
            enabled: true,
        }
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Get formatted bank details
    pub fn get_bank_details(&self) -> String {
        let mut details = Vec::new();

        if !self.config.bank_name.is_empty() {
            details.push(format!("Bank: {}", self.config.bank_name));
        }
        if !self.config.account_name.is_empty() {
            details.push(format!("Account Name: {}", self.config.account_name));
        }
        if !self.config.account_number.is_empty() {
            details.push(format!("Account Number: {}", self.config.account_number));
        }
        if !self.config.sort_code.is_empty() {
            details.push(format!("Sort Code: {}", self.config.sort_code));
        }
        if !self.config.iban.is_empty() {
            details.push(format!("IBAN: {}", self.config.iban));
        }
        if !self.config.bic.is_empty() {
            details.push(format!("BIC/SWIFT: {}", self.config.bic));
        }

        details.join("\n")
    }
}

#[async_trait]
impl PaymentGateway for BacsGateway {
    fn id(&self) -> &str {
        "bacs"
    }

    fn title(&self) -> &str {
        &self.config.title
    }

    fn description(&self) -> &str {
        &self.config.description
    }

    fn supports(&self) -> Vec<GatewayFeature> {
        vec![GatewayFeature::Products]
    }

    fn is_available(&self) -> bool {
        self.enabled
    }

    fn get_settings_fields(&self) -> Vec<GatewaySettingField> {
        vec![
            GatewaySettingField {
                id: "title".to_string(),
                title: "Title".to_string(),
                field_type: SettingFieldType::Text,
                description: Some("Payment method title shown to customers".to_string()),
                default: Some("Direct bank transfer".to_string()),
                options: vec![],
                required: true,
            },
            GatewaySettingField {
                id: "description".to_string(),
                title: "Description".to_string(),
                field_type: SettingFieldType::Textarea,
                description: Some("Payment method description shown to customers".to_string()),
                default: Some("Make your payment directly into our bank account.".to_string()),
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "instructions".to_string(),
                title: "Instructions".to_string(),
                field_type: SettingFieldType::Textarea,
                description: Some("Instructions shown on thank you page and in confirmation email".to_string()),
                default: None,
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "account_name".to_string(),
                title: "Account Name".to_string(),
                field_type: SettingFieldType::Text,
                description: None,
                default: None,
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "account_number".to_string(),
                title: "Account Number".to_string(),
                field_type: SettingFieldType::Text,
                description: None,
                default: None,
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "sort_code".to_string(),
                title: "Sort Code".to_string(),
                field_type: SettingFieldType::Text,
                description: None,
                default: None,
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "bank_name".to_string(),
                title: "Bank Name".to_string(),
                field_type: SettingFieldType::Text,
                description: None,
                default: None,
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "iban".to_string(),
                title: "IBAN".to_string(),
                field_type: SettingFieldType::Text,
                description: None,
                default: None,
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "bic".to_string(),
                title: "BIC/SWIFT".to_string(),
                field_type: SettingFieldType::Text,
                description: None,
                default: None,
                options: vec![],
                required: false,
            },
        ]
    }

    async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // BACS doesn't actually process payment - order waits for bank transfer
        let transaction_id = format!("BACS_{}", Uuid::now_v7());

        let bank_details = self.get_bank_details();
        let instructions = if self.config.instructions.is_empty() {
            format!(
                "Please make your payment to:\n\n{}\n\nPlease use your order number as the payment reference.",
                bank_details
            )
        } else {
            format!("{}\n\n{}", self.config.instructions, bank_details)
        };

        let mut result = PaymentResult::pending(transaction_id, Some(instructions));
        result.status = TransactionStatus::Pending;

        Ok(result)
    }

    async fn process_refund(&self, request: RefundRequest) -> Result<RefundResult, GatewayError> {
        // Bank transfer refunds are handled manually
        Ok(RefundResult {
            success: true,
            refund_id: Some(format!("BACS_REFUND_{}", Uuid::now_v7())),
            amount: request.amount.unwrap_or(Decimal::ZERO),
            message: Some("Bank transfer refund should be processed manually".to_string()),
            raw_response: None,
        })
    }

    fn get_icon_url(&self) -> Option<String> {
        Some("/plugins/rustcommerce/assets/images/bank.svg".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_id() {
        let gateway = BacsGateway::new(BacsConfig::default());
        assert_eq!(gateway.id(), "bacs");
    }

    #[test]
    fn test_bank_details_formatting() {
        let config = BacsConfig {
            bank_name: "Test Bank".to_string(),
            account_name: "My Store Ltd".to_string(),
            account_number: "12345678".to_string(),
            sort_code: "00-00-00".to_string(),
            ..Default::default()
        };
        let gateway = BacsGateway::new(config);

        let details = gateway.get_bank_details();
        assert!(details.contains("Test Bank"));
        assert!(details.contains("My Store Ltd"));
        assert!(details.contains("12345678"));
    }
}
