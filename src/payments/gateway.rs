//! Payment Gateway Base
//!
//! Defines the payment gateway trait and registry.

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::payment::{
    PaymentRequest, PaymentResult, RefundRequest, RefundResult,
    PaymentToken, GatewayFeature,
};
use crate::models::order::Order;

/// Payment gateway error
#[derive(Debug, Clone)]
pub enum GatewayError {
    NotConfigured,
    InvalidCredentials,
    NetworkError(String),
    PaymentDeclined(String),
    InvalidRequest(String),
    RefundFailed(String),
    UnsupportedFeature,
    RateLimited,
    UnknownError(String),
}

impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConfigured => write!(f, "Payment gateway not configured"),
            Self::InvalidCredentials => write!(f, "Invalid API credentials"),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::PaymentDeclined(msg) => write!(f, "Payment declined: {}", msg),
            Self::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            Self::RefundFailed(msg) => write!(f, "Refund failed: {}", msg),
            Self::UnsupportedFeature => write!(f, "Feature not supported"),
            Self::RateLimited => write!(f, "Rate limit exceeded"),
            Self::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for GatewayError {}

/// Payment gateway trait
#[async_trait]
pub trait PaymentGateway: Send + Sync {
    /// Get gateway ID
    fn id(&self) -> &str;

    /// Get gateway title
    fn title(&self) -> &str;

    /// Get gateway description
    fn description(&self) -> &str;

    /// Get supported features
    fn supports(&self) -> Vec<GatewayFeature>;

    /// Check if feature is supported
    fn supports_feature(&self, feature: GatewayFeature) -> bool {
        self.supports().contains(&feature)
    }

    /// Check if gateway is available
    fn is_available(&self) -> bool;

    /// Get settings fields for admin
    fn get_settings_fields(&self) -> Vec<GatewaySettingField>;

    /// Process a payment
    async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, GatewayError>;

    /// Process a refund
    async fn process_refund(&self, request: RefundRequest) -> Result<RefundResult, GatewayError>;

    /// Capture a previously authorized payment
    async fn capture(&self, transaction_id: &str, amount: Option<Decimal>) -> Result<PaymentResult, GatewayError> {
        Err(GatewayError::UnsupportedFeature)
    }

    /// Void a previously authorized payment
    async fn void(&self, transaction_id: &str) -> Result<PaymentResult, GatewayError> {
        Err(GatewayError::UnsupportedFeature)
    }

    /// Create a payment token for saved payment methods
    async fn create_token(&self, _request: TokenizeRequest) -> Result<PaymentToken, GatewayError> {
        Err(GatewayError::UnsupportedFeature)
    }

    /// Delete a payment token
    async fn delete_token(&self, _token_id: &str) -> Result<(), GatewayError> {
        Err(GatewayError::UnsupportedFeature)
    }

    /// Get saved payment methods for customer
    async fn get_saved_methods(&self, _customer_id: Uuid) -> Result<Vec<PaymentToken>, GatewayError> {
        Ok(vec![])
    }

    /// Handle webhook from payment provider
    async fn handle_webhook(&self, _payload: &[u8], _signature: Option<&str>) -> Result<WebhookResult, GatewayError> {
        Err(GatewayError::UnsupportedFeature)
    }

    /// Get icon URL
    fn get_icon_url(&self) -> Option<String> {
        None
    }
}

/// Gateway setting field
#[derive(Debug, Clone)]
pub struct GatewaySettingField {
    pub id: String,
    pub title: String,
    pub field_type: SettingFieldType,
    pub description: Option<String>,
    pub default: Option<String>,
    pub options: Vec<(String, String)>,
    pub required: bool,
}

/// Setting field types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingFieldType {
    Text,
    Password,
    Checkbox,
    Select,
    Textarea,
    Number,
}

/// Tokenize request
#[derive(Debug, Clone)]
pub struct TokenizeRequest {
    pub customer_id: Uuid,
    pub card_number: String,
    pub exp_month: String,
    pub exp_year: String,
    pub cvc: String,
    pub cardholder_name: Option<String>,
}

/// Webhook processing result
#[derive(Debug, Clone)]
pub struct WebhookResult {
    pub event_type: WebhookEventType,
    pub order_id: Option<Uuid>,
    pub transaction_id: Option<String>,
    pub message: Option<String>,
}

/// Webhook event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookEventType {
    PaymentCompleted,
    PaymentFailed,
    PaymentRefunded,
    PaymentDisputed,
    SubscriptionCreated,
    SubscriptionCancelled,
    SubscriptionRenewed,
    Unknown,
}

/// Payment gateway registry
pub struct PaymentGatewayRegistry {
    gateways: HashMap<String, Arc<dyn PaymentGateway>>,
}

impl PaymentGatewayRegistry {
    /// Create a new gateway registry
    pub fn new() -> Self {
        Self {
            gateways: HashMap::new(),
        }
    }

    /// Register a payment gateway
    pub fn register(&mut self, gateway: Arc<dyn PaymentGateway>) {
        self.gateways.insert(gateway.id().to_string(), gateway);
    }

    /// Get a gateway by ID
    pub fn get(&self, id: &str) -> Option<Arc<dyn PaymentGateway>> {
        self.gateways.get(id).cloned()
    }

    /// Get all registered gateways
    pub fn get_all(&self) -> Vec<Arc<dyn PaymentGateway>> {
        self.gateways.values().cloned().collect()
    }

    /// Get available gateways (configured and enabled)
    pub fn get_available(&self) -> Vec<Arc<dyn PaymentGateway>> {
        self.gateways.values()
            .filter(|g| g.is_available())
            .cloned()
            .collect()
    }

    /// Process payment using specified gateway
    pub async fn process_payment(
        &self,
        gateway_id: &str,
        request: PaymentRequest,
    ) -> Result<PaymentResult, GatewayError> {
        let gateway = self.get(gateway_id)
            .ok_or(GatewayError::NotConfigured)?;

        if !gateway.is_available() {
            return Err(GatewayError::NotConfigured);
        }

        gateway.process_payment(request).await
    }

    /// Process refund using specified gateway
    pub async fn process_refund(
        &self,
        gateway_id: &str,
        request: RefundRequest,
    ) -> Result<RefundResult, GatewayError> {
        let gateway = self.get(gateway_id)
            .ok_or(GatewayError::NotConfigured)?;

        gateway.process_refund(request).await
    }
}

impl Default for PaymentGatewayRegistry {
    fn default() -> Self {
        Self::new()
    }
}
