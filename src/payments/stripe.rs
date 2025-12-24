//! Stripe Payment Gateway
//!
//! Integration with Stripe for credit card and other payments.

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
    PaymentToken, PaymentTokenType, CardType, GatewayFeature,
    TransactionStatus,
};

/// Stripe gateway configuration
#[derive(Debug, Clone)]
pub struct StripeConfig {
    pub publishable_key: String,
    pub secret_key: String,
    pub webhook_secret: Option<String>,
    pub testmode: bool,
    pub capture: bool, // Capture immediately or authorize only
    pub payment_request_button: bool,
    pub saved_cards: bool,
}

impl Default for StripeConfig {
    fn default() -> Self {
        Self {
            publishable_key: String::new(),
            secret_key: String::new(),
            webhook_secret: None,
            testmode: true,
            capture: true,
            payment_request_button: true,
            saved_cards: true,
        }
    }
}

/// Stripe payment gateway
pub struct StripeGateway {
    config: StripeConfig,
    enabled: bool,
}

impl StripeGateway {
    /// Create a new Stripe gateway
    pub fn new(config: StripeConfig) -> Self {
        Self {
            config,
            enabled: true,
        }
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Get API base URL
    fn api_url(&self) -> &str {
        "https://api.stripe.com/v1"
    }

    /// Convert amount to smallest currency unit (cents)
    fn to_smallest_unit(&self, amount: Decimal, currency: &str) -> i64 {
        // Most currencies use 2 decimal places
        let multiplier = match currency.to_uppercase().as_str() {
            "JPY" | "KRW" | "VND" => 1,
            "BHD" | "KWD" | "OMR" => 1000,
            _ => 100,
        };
        (amount * Decimal::from(multiplier)).to_string().parse().unwrap_or(0)
    }

    /// Create payment intent
    async fn create_payment_intent(
        &self,
        request: &PaymentRequest,
    ) -> Result<StripePaymentIntent, GatewayError> {
        // In production, this would make actual HTTP request to Stripe API
        // For now, simulate a successful payment intent creation

        if self.config.secret_key.is_empty() {
            return Err(GatewayError::InvalidCredentials);
        }

        let intent_id = format!("pi_{}", Uuid::now_v7().to_string().replace("-", ""));

        Ok(StripePaymentIntent {
            id: intent_id.clone(),
            client_secret: format!("{}_secret_{}", intent_id, Uuid::now_v7()),
            status: "succeeded".to_string(),
            amount: self.to_smallest_unit(request.amount, &request.currency),
            currency: request.currency.to_lowercase(),
        })
    }

    /// Confirm payment intent
    async fn confirm_payment_intent(
        &self,
        intent_id: &str,
    ) -> Result<StripePaymentIntent, GatewayError> {
        // Simulate confirmation
        Ok(StripePaymentIntent {
            id: intent_id.to_string(),
            client_secret: String::new(),
            status: "succeeded".to_string(),
            amount: 0,
            currency: "usd".to_string(),
        })
    }
}

#[async_trait]
impl PaymentGateway for StripeGateway {
    fn id(&self) -> &str {
        "stripe"
    }

    fn title(&self) -> &str {
        "Credit Card (Stripe)"
    }

    fn description(&self) -> &str {
        "Pay with your credit card via Stripe."
    }

    fn supports(&self) -> Vec<GatewayFeature> {
        vec![
            GatewayFeature::Products,
            GatewayFeature::Subscriptions,
            GatewayFeature::Refunds,
            GatewayFeature::Tokenization,
            GatewayFeature::SavePaymentMethod,
            GatewayFeature::ApplePay,
            GatewayFeature::GooglePay,
            GatewayFeature::ThreeDSecure,
            GatewayFeature::MultiCurrency,
        ]
    }

    fn is_available(&self) -> bool {
        self.enabled &&
        !self.config.publishable_key.is_empty() &&
        !self.config.secret_key.is_empty()
    }

    fn get_settings_fields(&self) -> Vec<GatewaySettingField> {
        vec![
            GatewaySettingField {
                id: "testmode".to_string(),
                title: "Test mode".to_string(),
                field_type: SettingFieldType::Checkbox,
                description: Some("Enable test mode for testing payments".to_string()),
                default: Some("yes".to_string()),
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "publishable_key".to_string(),
                title: "Publishable Key".to_string(),
                field_type: SettingFieldType::Text,
                description: Some("Your Stripe publishable key".to_string()),
                default: None,
                options: vec![],
                required: true,
            },
            GatewaySettingField {
                id: "secret_key".to_string(),
                title: "Secret Key".to_string(),
                field_type: SettingFieldType::Password,
                description: Some("Your Stripe secret key".to_string()),
                default: None,
                options: vec![],
                required: true,
            },
            GatewaySettingField {
                id: "webhook_secret".to_string(),
                title: "Webhook Secret".to_string(),
                field_type: SettingFieldType::Password,
                description: Some("Your Stripe webhook signing secret".to_string()),
                default: None,
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "capture".to_string(),
                title: "Capture".to_string(),
                field_type: SettingFieldType::Checkbox,
                description: Some("Capture payments immediately".to_string()),
                default: Some("yes".to_string()),
                options: vec![],
                required: false,
            },
            GatewaySettingField {
                id: "saved_cards".to_string(),
                title: "Saved Cards".to_string(),
                field_type: SettingFieldType::Checkbox,
                description: Some("Allow customers to save cards for future purchases".to_string()),
                default: Some("yes".to_string()),
                options: vec![],
                required: false,
            },
        ]
    }

    async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // Create payment intent
        let intent = self.create_payment_intent(&request).await?;

        // Check if 3DS is required
        if intent.status == "requires_action" {
            return Ok(PaymentResult::requires_action(
                format!("https://stripe.com/3ds?intent={}", intent.id)
            ));
        }

        // Check status
        match intent.status.as_str() {
            "succeeded" => {
                Ok(PaymentResult::success(intent.id))
            }
            "requires_capture" => {
                // Authorization only
                let mut result = PaymentResult::success(intent.id);
                result.status = TransactionStatus::Pending;
                result.message = Some("Payment authorized, awaiting capture".to_string());
                Ok(result)
            }
            "requires_payment_method" | "requires_confirmation" => {
                Ok(PaymentResult::failure("Payment requires additional information".to_string()))
            }
            _ => {
                Ok(PaymentResult::failure(format!("Payment failed: {}", intent.status)))
            }
        }
    }

    async fn process_refund(&self, request: RefundRequest) -> Result<RefundResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // In production, would call Stripe refund API
        let refund_id = format!("re_{}", Uuid::now_v7().to_string().replace("-", ""));

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

        // In production, would call Stripe capture API
        Ok(PaymentResult::success(transaction_id.to_string()))
    }

    async fn void(&self, transaction_id: &str) -> Result<PaymentResult, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // In production, would call Stripe cancel API
        Ok(PaymentResult::success(transaction_id.to_string()))
    }

    async fn create_token(&self, request: TokenizeRequest) -> Result<PaymentToken, GatewayError> {
        if !self.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        // Detect card type
        let card_type = CardType::from_number(&request.card_number);
        let last_four = request.card_number.chars().rev().take(4).collect::<String>().chars().rev().collect();

        // In production, would create Stripe customer and payment method
        let token_id = format!("pm_{}", Uuid::now_v7().to_string().replace("-", ""));

        Ok(PaymentToken {
            id: Uuid::now_v7(),
            site_id: None,
            customer_id: request.customer_id,
            gateway_id: "stripe".to_string(),
            token: token_id,
            token_type: PaymentTokenType::CreditCard,
            last_four: Some(last_four),
            expiry_month: Some(request.exp_month),
            expiry_year: Some(request.exp_year),
            card_type,
            is_default: false,
            created_at: chrono::Utc::now(),
            expires_at: None,
        })
    }

    async fn handle_webhook(&self, payload: &[u8], signature: Option<&str>) -> Result<WebhookResult, GatewayError> {
        // Verify webhook signature
        if let Some(secret) = &self.config.webhook_secret {
            if signature.is_none() {
                return Err(GatewayError::InvalidRequest("Missing webhook signature".to_string()));
            }
            // In production, verify HMAC signature
        }

        // Parse webhook payload
        let event: StripeWebhookEvent = serde_json::from_slice(payload)
            .map_err(|e| GatewayError::InvalidRequest(format!("Invalid webhook payload: {}", e)))?;

        let event_type = match event.event_type.as_str() {
            "payment_intent.succeeded" => WebhookEventType::PaymentCompleted,
            "payment_intent.payment_failed" => WebhookEventType::PaymentFailed,
            "charge.refunded" => WebhookEventType::PaymentRefunded,
            "charge.dispute.created" => WebhookEventType::PaymentDisputed,
            "customer.subscription.created" => WebhookEventType::SubscriptionCreated,
            "customer.subscription.deleted" => WebhookEventType::SubscriptionCancelled,
            "invoice.paid" => WebhookEventType::SubscriptionRenewed,
            _ => WebhookEventType::Unknown,
        };

        Ok(WebhookResult {
            event_type,
            order_id: None, // Would parse from metadata
            transaction_id: event.data.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            message: Some(event.event_type),
        })
    }

    fn get_icon_url(&self) -> Option<String> {
        Some("/plugins/rustcommerce/assets/images/stripe.svg".to_string())
    }
}

/// Stripe payment intent response
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StripePaymentIntent {
    id: String,
    client_secret: String,
    status: String,
    amount: i64,
    currency: String,
}

/// Stripe webhook event
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StripeWebhookEvent {
    #[serde(rename = "type")]
    event_type: String,
    data: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_id() {
        let gateway = StripeGateway::new(StripeConfig::default());
        assert_eq!(gateway.id(), "stripe");
    }

    #[test]
    fn test_supports_refunds() {
        let gateway = StripeGateway::new(StripeConfig::default());
        assert!(gateway.supports_feature(GatewayFeature::Refunds));
    }

    #[test]
    fn test_not_available_without_keys() {
        let gateway = StripeGateway::new(StripeConfig::default());
        assert!(!gateway.is_available());
    }

    #[test]
    fn test_available_with_keys() {
        let config = StripeConfig {
            publishable_key: "pk_test_xxx".to_string(),
            secret_key: "sk_test_xxx".to_string(),
            ..Default::default()
        };
        let gateway = StripeGateway::new(config);
        assert!(gateway.is_available());
    }

    #[test]
    fn test_to_smallest_unit() {
        let gateway = StripeGateway::new(StripeConfig::default());
        assert_eq!(gateway.to_smallest_unit(Decimal::from(10), "USD"), 1000);
        assert_eq!(gateway.to_smallest_unit(Decimal::from(10), "JPY"), 10);
    }
}
