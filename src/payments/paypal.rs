//! PayPal Payment Gateway
//!
//! Integration with PayPal for PayPal and credit card payments.

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::gateway::{
    PaymentGateway, GatewayError, GatewaySettingField, SettingFieldType,
    TokenizeRequest, WebhookResult, WebhookEventType,
};
use crate::models::payment::{
    PaymentRequest, PaymentResult, RefundRequest, RefundResult,
    PaymentToken, PaymentTokenType, GatewayFeature, TransactionStatus,
};

/// PayPal gateway configuration
#[derive(Debug, Clone)]
pub struct PayPalConfig {
    pub client_id: String,
    pub client_secret: String,
    pub sandbox: bool,
    pub intent: PayPalIntent,
    pub disable_funding: Vec<String>,
    pub button_color: PayPalButtonColor,
    pub button_shape: PayPalButtonShape,
}

impl Default for PayPalConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            sandbox: true,
            intent: PayPalIntent::Capture,
            disable_funding: vec![],
            button_color: PayPalButtonColor::Gold,
            button_shape: PayPalButtonShape::Rect,
        }
    }
}

/// PayPal payment intent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayPalIntent {
    Capture,
    Authorize,
}

/// PayPal button color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayPalButtonColor {
    Gold,
    Blue,
    Silver,
    White,
    Black,
}

/// PayPal button shape
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayPalButtonShape {
    Rect,
    Pill,
}

/// PayPal payment gateway
pub struct PayPalGateway {
    config: PayPalConfig,
    enabled: bool,
    access_token: Option<String>,
}

impl PayPalGateway {
    /// Create a new PayPal gateway
    pub fn new(config: PayPalConfig) -> Self {
        Self {
            config,
            enabled: true,
            access_token: None,
        }
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Get API base URL
    fn api_url(&self) -> &str {
        if self.config.sandbox {
            "https://api-m.sandbox.paypal.com"
        } else {
            "https://api-m.paypal.com"
        }
    }

    /// Get OAuth2 access token
    async fn get_access_token(&self) -> Result<String, GatewayError> {
        // In production, this would make OAuth2 token request
        // For now, simulate token generation
        if self.config.client_id.is_empty() || self.config.client_secret.is_empty() {
            return Err(GatewayError::InvalidCredentials);
        }

        Ok(format!("access_token_{}", Uuid::now_v7()))
    }

    /// Create PayPal order
    async fn create_order(&self, request: &PaymentRequest) -> Result<PayPalOrder, GatewayError> {
        let _token = self.get_access_token().await?;

        // In production, would create order via PayPal API
        let order_id = format!("PAYPAL_{}", Uuid::now_v7().to_string().replace("-", "").to_uppercase());

        Ok(PayPalOrder {
            id: order_id.clone(),
            status: "CREATED".to_string(),
            approve_url: format!("{}/checkoutnow?token={}", self.api_url(), order_id),
        })
    }

    /// Capture PayPal order
    async fn capture_order(&self, order_id: &str) -> Result<PayPalCapture, GatewayError> {
        let _token = self.get_access_token().await?;

        // In production, would capture via PayPal API
        Ok(PayPalCapture {
            id: format!("CAPTURE_{}", Uuid::now_v7()),
            status: "COMPLETED".to_string(),
            amount: PayPalAmount {
                currency_code: "USD".to_string(),
                value: "0.00".to_string(),
            },
        })
    }
}

#[async_trait]
impl PaymentGateway for PayPalGateway {
    fn id(&self) -> &str {
        "paypal"
    }

    fn title(&self) -> &str {
        "PayPal"
    }

    fn description(&self) -> &str {
        "Pay via PayPal; you can pay with your credit card if you don't have a PayPal account."
    }

    fn supports(&self) -> Vec<GatewayFeature> {
        vec![
            GatewayFeature::Products,
            GatewayFeature::Subscriptions,
            GatewayFeature::Refunds,
            GatewayFeature::MultiCurrency,
        ]
    }

    fn is_available(&self) -> bool {
        self.enabled &&
        !self.config.client_id.is_empty() &&
        !self.config.client_secret.is_empty()
    }

    fn get_settings_fields(&self) -> Vec<GatewaySettingField> {
        vec![
            GatewaySettingField {
                id: "sandbox".to_string(),
                title: "Sandbox Mode".to_string(),
                field_type: SettingFieldType::Checkbox,
                description: Some("Enable sandbox mode for testing".to_string()),
                default: Some("yes".to_string()),
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "client_id".to_string(),
                title: "Client ID".to_string(),
                field_type: SettingFieldType::Text,
                description: Some("Your PayPal REST API client ID".to_string()),
                default: None,
                options: vec![],
                required: true,
            },
            GatewaySettingField {
                id: "client_secret".to_string(),
                title: "Client Secret".to_string(),
                field_type: SettingFieldType::Password,
                description: Some("Your PayPal REST API client secret".to_string()),
                default: None,
                options: vec![],
                required: true,
            },
            GatewaySettingField {
                id: "intent".to_string(),
                title: "Payment Action".to_string(),
                field_type: SettingFieldType::Select,
                description: Some("Choose whether to capture funds immediately or authorize only".to_string()),
                default: Some("capture".to_string()),
                options: vec![
                    ("capture".to_string(), "Capture".to_string()),
                    ("authorize".to_string(), "Authorize".to_string()),
                ],
                required: false,
            },
            GatewaySettingField {
                id: "button_color".to_string(),
                title: "Button Color".to_string(),
                field_type: SettingFieldType::Select,
                description: Some("PayPal button color".to_string()),
                default: Some("gold".to_string()),
                options: vec![
                    ("gold".to_string(), "Gold".to_string()),
                    ("blue".to_string(), "Blue".to_string()),
                    ("silver".to_string(), "Silver".to_string()),
                    ("white".to_string(), "White".to_string()),
                    ("black".to_string(), "Black".to_string()),
                ],
                required: false,
            },
        ]
    }

    async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // Create PayPal order - user will be redirected to approve
        let order = self.create_order(&request).await?;

        // PayPal payments require user redirect
        let mut result = PaymentResult::pending(order.id, Some("Awaiting PayPal approval".to_string()));
        result.redirect_url = Some(order.approve_url);
        result.requires_action = true;

        Ok(result)
    }

    async fn process_refund(&self, request: RefundRequest) -> Result<RefundResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        let _token = self.get_access_token().await?;

        // In production, would call PayPal refund API
        let refund_id = format!("REFUND_{}", Uuid::now_v7());

        Ok(RefundResult {
            success: true,
            refund_id: Some(refund_id),
            amount: request.amount.unwrap_or(Decimal::ZERO),
            message: Some("Refund processed successfully".to_string()),
            raw_response: None,
        })
    }

    async fn capture(&self, transaction_id: &str, amount: Option<Decimal>) -> Result<PaymentResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        let capture = self.capture_order(transaction_id).await?;

        if capture.status == "COMPLETED" {
            Ok(PaymentResult::success(capture.id))
        } else {
            Ok(PaymentResult::failure(format!("Capture failed: {}", capture.status)))
        }
    }

    async fn void(&self, transaction_id: &str) -> Result<PaymentResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // In production, would void authorization via PayPal API
        Ok(PaymentResult::success(transaction_id.to_string()))
    }

    async fn handle_webhook(&self, payload: &[u8], signature: Option<&str>) -> Result<WebhookResult, GatewayError> {
        // Parse webhook payload
        let event: PayPalWebhookEvent = serde_json::from_slice(payload)
            .map_err(|e| GatewayError::InvalidRequest(format!("Invalid webhook payload: {}", e)))?;

        let event_type = match event.event_type.as_str() {
            "PAYMENT.CAPTURE.COMPLETED" => WebhookEventType::PaymentCompleted,
            "PAYMENT.CAPTURE.DENIED" => WebhookEventType::PaymentFailed,
            "PAYMENT.CAPTURE.REFUNDED" => WebhookEventType::PaymentRefunded,
            "CUSTOMER.DISPUTE.CREATED" => WebhookEventType::PaymentDisputed,
            "BILLING.SUBSCRIPTION.CREATED" => WebhookEventType::SubscriptionCreated,
            "BILLING.SUBSCRIPTION.CANCELLED" => WebhookEventType::SubscriptionCancelled,
            "BILLING.SUBSCRIPTION.PAYMENT.FAILED" => WebhookEventType::PaymentFailed,
            _ => WebhookEventType::Unknown,
        };

        Ok(WebhookResult {
            event_type,
            order_id: None,
            transaction_id: event.resource.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            message: Some(event.event_type),
        })
    }

    fn get_icon_url(&self) -> Option<String> {
        Some("/plugins/rustcommerce/assets/images/paypal.svg".to_string())
    }
}

/// PayPal order response
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PayPalOrder {
    id: String,
    status: String,
    approve_url: String,
}

/// PayPal capture response
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PayPalCapture {
    id: String,
    status: String,
    amount: PayPalAmount,
}

/// PayPal amount
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PayPalAmount {
    currency_code: String,
    value: String,
}

/// PayPal webhook event
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PayPalWebhookEvent {
    event_type: String,
    resource: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_id() {
        let gateway = PayPalGateway::new(PayPalConfig::default());
        assert_eq!(gateway.id(), "paypal");
    }

    #[test]
    fn test_sandbox_url() {
        let gateway = PayPalGateway::new(PayPalConfig::default());
        assert_eq!(gateway.api_url(), "https://api-m.sandbox.paypal.com");
    }

    #[test]
    fn test_production_url() {
        let config = PayPalConfig {
            sandbox: false,
            ..Default::default()
        };
        let gateway = PayPalGateway::new(config);
        assert_eq!(gateway.api_url(), "https://api-m.paypal.com");
    }

    #[test]
    fn test_not_available_without_credentials() {
        let gateway = PayPalGateway::new(PayPalConfig::default());
        assert!(!gateway.is_available());
    }
}
