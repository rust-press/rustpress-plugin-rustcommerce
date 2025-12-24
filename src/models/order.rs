//! Order Models

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::customer::Address;

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    #[default]
    Pending,
    Processing,
    OnHold,
    Completed,
    Cancelled,
    Refunded,
    Failed,
    CheckoutDraft,
}

impl OrderStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Pending => "Pending Payment",
            Self::Processing => "Processing",
            Self::OnHold => "On Hold",
            Self::Completed => "Completed",
            Self::Cancelled => "Cancelled",
            Self::Refunded => "Refunded",
            Self::Failed => "Failed",
            Self::CheckoutDraft => "Draft",
        }
    }

    pub fn is_paid(&self) -> bool {
        matches!(self, Self::Processing | Self::Completed)
    }

    pub fn can_cancel(&self) -> bool {
        matches!(self, Self::Pending | Self::OnHold | Self::Failed)
    }

    pub fn can_refund(&self) -> bool {
        matches!(self, Self::Processing | Self::Completed)
    }
}

/// Order entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub order_number: String,

    // Customer
    pub customer_id: Option<Uuid>,
    pub customer_ip_address: Option<String>,
    pub customer_user_agent: Option<String>,

    // Status
    pub status: OrderStatus,
    pub parent_id: Option<Uuid>,

    // Currency
    pub currency: String,
    pub currency_symbol: String,

    // Prices
    pub prices_include_tax: bool,

    // Totals
    pub discount_total: Decimal,
    pub discount_tax: Decimal,
    pub shipping_total: Decimal,
    pub shipping_tax: Decimal,
    pub cart_tax: Decimal,
    pub total: Decimal,
    pub total_tax: Decimal,

    // Addresses
    pub billing: Address,
    pub shipping: Address,

    // Payment
    pub payment_method: Option<String>,
    pub payment_method_title: Option<String>,
    pub transaction_id: Option<String>,

    // Shipping method
    pub shipping_method: Option<String>,
    pub shipping_method_title: Option<String>,

    // Notes
    pub customer_note: Option<String>,

    // Dates
    pub date_paid: Option<DateTime<Utc>>,
    pub date_completed: Option<DateTime<Utc>>,

    // Cart hash (for duplicate detection)
    pub cart_hash: Option<String>,

    // Metadata
    pub meta: serde_json::Value,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,

    // Related data (loaded separately)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<OrderItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_lines: Option<Vec<OrderShippingLine>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_lines: Option<Vec<OrderTaxLine>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_lines: Option<Vec<OrderFeeLine>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_lines: Option<Vec<OrderCouponLine>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<OrderNote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<OrderRefund>>,
}

impl Order {
    /// Calculate subtotal (before discounts, shipping, tax)
    pub fn get_subtotal(&self) -> Decimal {
        self.line_items
            .as_ref()
            .map(|items| items.iter().map(|i| i.subtotal).sum())
            .unwrap_or(Decimal::ZERO)
    }

    /// Get the formatted order number
    pub fn get_formatted_number(&self) -> String {
        format!("#{}", self.order_number)
    }

    /// Check if order needs payment
    pub fn needs_payment(&self) -> bool {
        self.status == OrderStatus::Pending && self.total > Decimal::ZERO
    }

    /// Check if order is editable
    pub fn is_editable(&self) -> bool {
        matches!(self.status, OrderStatus::Pending | OrderStatus::OnHold | OrderStatus::CheckoutDraft)
    }

    /// Get customer name
    pub fn get_customer_name(&self) -> String {
        format!("{} {}", self.billing.first_name, self.billing.last_name)
            .trim()
            .to_string()
    }
}

/// Order item type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderItemType {
    #[default]
    LineItem,
    Shipping,
    Tax,
    Coupon,
    Fee,
}

/// Order line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub item_type: OrderItemType,
    pub name: String,
    pub quantity: i32,
    pub subtotal: Decimal,
    pub subtotal_tax: Decimal,
    pub total: Decimal,
    pub total_tax: Decimal,

    // Product reference
    pub product_id: Option<Uuid>,
    pub variation_id: Option<Uuid>,
    pub sku: Option<String>,

    // Metadata
    pub meta: serde_json::Value,
    pub created_at: DateTime<Utc>,

    // Additional data (for display)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_attributes: Option<Vec<ItemAttribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAttribute {
    pub name: String,
    pub value: String,
}

/// Order shipping line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderShippingLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub method_id: String,
    pub method_title: String,
    pub instance_id: Option<String>,
    pub total: Decimal,
    pub total_tax: Decimal,
    pub taxes: Vec<OrderItemTax>,
    pub meta: serde_json::Value,
}

/// Order tax line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTaxLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub rate_id: Uuid,
    pub rate_code: String,
    pub label: String,
    pub compound: bool,
    pub tax_total: Decimal,
    pub shipping_tax_total: Decimal,
}

/// Order item tax breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemTax {
    pub rate_id: Uuid,
    pub total: Decimal,
    pub subtotal: Decimal,
}

/// Order fee line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFeeLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub name: String,
    pub tax_class: String,
    pub tax_status: String,
    pub amount: Decimal,
    pub total: Decimal,
    pub total_tax: Decimal,
}

/// Order coupon line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCouponLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub code: String,
    pub discount: Decimal,
    pub discount_tax: Decimal,
    pub discount_type: String,
    pub coupon_id: Option<Uuid>,
}

/// Order note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderNote {
    pub id: Uuid,
    pub order_id: Uuid,
    pub content: String,
    pub is_customer_note: bool,
    pub added_by_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Order refund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRefund {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub refunded_by: Option<Uuid>,
    pub refunded_payment: bool,
    pub created_at: DateTime<Utc>,
    pub items: Option<Vec<RefundItem>>,
}

/// Refund item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItem {
    pub id: Uuid,
    pub refund_id: Uuid,
    pub order_item_id: Uuid,
    pub quantity: i32,
    pub refund_total: Decimal,
    pub refund_tax: Decimal,
}

// =============================================================================
// DTOs for API
// =============================================================================

/// Request to create an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub status: Option<OrderStatus>,
    pub customer_id: Option<Uuid>,
    pub billing: Address,
    pub shipping: Option<Address>,
    pub payment_method: Option<String>,
    pub payment_method_title: Option<String>,
    pub transaction_id: Option<String>,
    pub customer_note: Option<String>,
    pub line_items: Vec<CreateOrderItemRequest>,
    pub shipping_lines: Option<Vec<CreateShippingLineRequest>>,
    pub fee_lines: Option<Vec<CreateFeeLineRequest>>,
    pub coupon_lines: Option<Vec<CreateCouponLineRequest>>,
    pub set_paid: Option<bool>,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: i32,
    pub subtotal: Option<Decimal>,
    pub total: Option<Decimal>,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShippingLineRequest {
    pub method_id: String,
    pub method_title: String,
    pub total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeeLineRequest {
    pub name: String,
    pub total: Decimal,
    pub tax_class: Option<String>,
    pub tax_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouponLineRequest {
    pub code: String,
}

/// Request to update an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub status: Option<OrderStatus>,
    pub billing: Option<Address>,
    pub shipping: Option<Address>,
    pub payment_method: Option<String>,
    pub payment_method_title: Option<String>,
    pub transaction_id: Option<String>,
    pub customer_note: Option<String>,
    pub meta: Option<serde_json::Value>,
}

/// Request to create a refund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    pub amount: Decimal,
    pub reason: Option<String>,
    pub refund_payment: bool,
    pub items: Option<Vec<CreateRefundItemRequest>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundItemRequest {
    pub order_item_id: Uuid,
    pub quantity: i32,
    pub refund_total: Decimal,
    pub refund_tax: Option<Decimal>,
}

/// Order filter/query parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderFilter {
    pub status: Option<Vec<OrderStatus>>,
    pub customer_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub search: Option<String>,
    pub include: Option<Vec<Uuid>>,
    pub exclude: Option<Vec<Uuid>>,
    pub orderby: Option<OrderOrderBy>,
    pub order: Option<super::product::SortOrder>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderOrderBy {
    #[default]
    Date,
    Id,
    Total,
    OrderNumber,
}
