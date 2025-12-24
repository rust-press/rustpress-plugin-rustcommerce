//! Cash on Delivery Payment Gateway
//!
//! Payment method for cash on delivery orders.

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

/// COD gateway configuration
#[derive(Debug, Clone)]
pub struct CodConfig {
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub enable_for_shipping_methods: Vec<String>,
    pub enable_for_virtual: bool,
    pub default_order_status: String,
}

impl Default for CodConfig {
    fn default() -> Self {
        Self {
            title: "Cash on delivery".to_string(),
            description: "Pay with cash upon delivery.".to_string(),
            instructions: "Pay with cash upon delivery.".to_string(),
            enable_for_shipping_methods: vec![],
            enable_for_virtual: false,
            default_order_status: "on-hold".to_string(),
        }
    }
}

/// Cash on Delivery payment gateway
pub struct CodGateway {
    config: CodConfig,
    enabled: bool,
}

impl CodGateway {
    /// Create a new COD gateway
    pub fn new(config: CodConfig) -> Self {
        Self {
            config,
            enabled: true,
        }
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[async_trait]
impl PaymentGateway for CodGateway {
    fn id(&self) -> &str {
        "cod"
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
                default: Some("Cash on delivery".to_string()),
                options: vec![],
                required: true,
            },
            GatewaySettingField {
                id: "description".to_string(),
                title: "Description".to_string(),
                field_type: SettingFieldType::Textarea,
                description: Some("Payment method description shown to customers".to_string()),
                default: Some("Pay with cash upon delivery.".to_string()),
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "instructions".to_string(),
                title: "Instructions".to_string(),
                field_type: SettingFieldType::Textarea,
                description: Some("Instructions shown on thank you page and in confirmation email".to_string()),
                default: Some("Pay with cash upon delivery.".to_string()),
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "enable_for_virtual".to_string(),
                title: "Accept for virtual orders".to_string(),
                field_type: SettingFieldType::Checkbox,
                description: Some("Accept COD for orders with only virtual products".to_string()),
                default: Some("no".to_string()),
                options: vec![],
                required: false,
            },
        ]
    }

    async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // COD doesn't actually process payment - just marks as pending
        let transaction_id = format!("COD_{}", Uuid::now_v7());

        let mut result = PaymentResult::pending(
            transaction_id,
            Some(self.config.instructions.clone()),
        );
        result.status = TransactionStatus::Pending;

        Ok(result)
    }

    async fn process_refund(&self, request: RefundRequest) -> Result<RefundResult, GatewayError> {
        // COD refunds are handled manually
        Ok(RefundResult {
            success: true,
            refund_id: Some(format!("COD_REFUND_{}", Uuid::now_v7())),
            amount: request.amount.unwrap_or(Decimal::ZERO),
            message: Some("Cash refund should be processed manually".to_string()),
            raw_response: None,
        })
    }

    fn get_icon_url(&self) -> Option<String> {
        Some("/plugins/rustcommerce/assets/images/cod.svg".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_id() {
        let gateway = CodGateway::new(CodConfig::default());
        assert_eq!(gateway.id(), "cod");
    }

    #[test]
    fn test_is_available() {
        let gateway = CodGateway::new(CodConfig::default());
        assert!(gateway.is_available());
    }

    #[test]
    fn test_no_tokenization_support() {
        let gateway = CodGateway::new(CodConfig::default());
        assert!(!gateway.supports_feature(GatewayFeature::Tokenization));
    }
}
