//! Order Service
//!
//! Handles order management, status transitions, and order operations.

use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::order::{
    Order, OrderItem, OrderStatus, OrderNote, OrderRefund,
    OrderShippingLine, OrderTaxLine, OrderFeeLine
};
use crate::settings::RustCommerceSettings;

/// Order service
pub struct OrderService {
    settings: RustCommerceSettings,
}

/// Order status transition
#[derive(Debug, Clone)]
pub struct StatusTransition {
    pub from: OrderStatus,
    pub to: OrderStatus,
    pub timestamp: DateTime<Utc>,
    pub note: Option<String>,
    pub user_id: Option<Uuid>,
}

/// Order search filter
#[derive(Debug, Clone, Default)]
pub struct OrderFilter {
    pub status: Option<Vec<OrderStatus>>,
    pub customer_id: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub min_total: Option<Decimal>,
    pub max_total: Option<Decimal>,
    pub search: Option<String>,
    pub product_id: Option<Uuid>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub order_by: Option<String>,
    pub order: Option<String>,
}

/// Order operation result
#[derive(Debug)]
pub enum OrderResult<T> {
    Success(T),
    Error(OrderError),
}

/// Order errors
#[derive(Debug, Clone)]
pub enum OrderError {
    NotFound,
    InvalidStatusTransition { from: OrderStatus, to: OrderStatus },
    AlreadyPaid,
    CannotRefund(String),
    InvalidAmount,
    OrderLocked,
}

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Order not found"),
            Self::InvalidStatusTransition { from, to } => {
                write!(f, "Cannot transition from {:?} to {:?}", from, to)
            }
            Self::AlreadyPaid => write!(f, "Order is already paid"),
            Self::CannotRefund(msg) => write!(f, "Cannot refund: {}", msg),
            Self::InvalidAmount => write!(f, "Invalid amount"),
            Self::OrderLocked => write!(f, "Order is locked and cannot be modified"),
        }
    }
}

impl OrderService {
    /// Create a new order service
    pub fn new(settings: RustCommerceSettings) -> Self {
        Self { settings }
    }

    /// Get valid status transitions for a given status
    pub fn get_valid_transitions(&self, status: OrderStatus) -> Vec<OrderStatus> {
        match status {
            OrderStatus::Pending => vec![
                OrderStatus::Processing,
                OrderStatus::OnHold,
                OrderStatus::Cancelled,
                OrderStatus::Failed,
            ],
            OrderStatus::Processing => vec![
                OrderStatus::Completed,
                OrderStatus::OnHold,
                OrderStatus::Cancelled,
                OrderStatus::Refunded,
            ],
            OrderStatus::OnHold => vec![
                OrderStatus::Pending,
                OrderStatus::Processing,
                OrderStatus::Cancelled,
            ],
            OrderStatus::Completed => vec![
                OrderStatus::Refunded,
            ],
            OrderStatus::Cancelled => vec![
                OrderStatus::Pending,
            ],
            OrderStatus::Refunded => vec![],
            OrderStatus::Failed => vec![
                OrderStatus::Pending,
            ],
            OrderStatus::Draft => vec![
                OrderStatus::Pending,
            ],
            OrderStatus::Checkout => vec![
                OrderStatus::Pending,
                OrderStatus::Failed,
            ],
        }
    }

    /// Check if status transition is valid
    pub fn can_transition(&self, from: OrderStatus, to: OrderStatus) -> bool {
        self.get_valid_transitions(from).contains(&to)
    }

    /// Update order status
    pub fn update_status(
        &self,
        order: &mut Order,
        new_status: OrderStatus,
        note: Option<String>,
    ) -> Result<StatusTransition, OrderError> {
        let old_status = order.status;

        if old_status == new_status {
            return Ok(StatusTransition {
                from: old_status,
                to: new_status,
                timestamp: Utc::now(),
                note,
                user_id: None,
            });
        }

        if !self.can_transition(old_status, new_status) {
            return Err(OrderError::InvalidStatusTransition {
                from: old_status,
                to: new_status,
            });
        }

        // Update timestamps based on status
        let now = Utc::now();
        order.status = new_status;
        order.updated_at = Some(now);

        match new_status {
            OrderStatus::Processing => {
                if order.paid_at.is_none() {
                    order.paid_at = Some(now);
                }
            }
            OrderStatus::Completed => {
                order.completed_at = Some(now);
            }
            _ => {}
        }

        Ok(StatusTransition {
            from: old_status,
            to: new_status,
            timestamp: now,
            note,
            user_id: None,
        })
    }

    /// Add note to order
    pub fn add_note(
        &self,
        order_id: Uuid,
        content: String,
        is_customer_note: bool,
        added_by: Option<String>,
    ) -> OrderNote {
        OrderNote {
            id: Uuid::now_v7(),
            order_id,
            content,
            is_customer_note,
            added_by,
            created_at: Utc::now(),
        }
    }

    /// Create a refund
    pub fn create_refund(
        &self,
        order: &Order,
        amount: Decimal,
        reason: Option<String>,
        items: Vec<(Uuid, i32)>, // (item_id, quantity)
        restock_items: bool,
    ) -> Result<OrderRefund, OrderError> {
        // Validate order can be refunded
        if !self.can_refund(order) {
            return Err(OrderError::CannotRefund(
                "Order status does not allow refunds".to_string()
            ));
        }

        // Validate amount
        let already_refunded = self.get_total_refunded(order);
        let max_refundable = order.total - already_refunded;

        if amount <= Decimal::ZERO {
            return Err(OrderError::InvalidAmount);
        }

        if amount > max_refundable {
            return Err(OrderError::CannotRefund(
                format!("Maximum refundable amount is {}", max_refundable)
            ));
        }

        Ok(OrderRefund {
            id: Uuid::now_v7(),
            order_id: order.id,
            amount,
            reason,
            refunded_by: None,
            created_at: Utc::now(),
        })
    }

    /// Check if order can be refunded
    pub fn can_refund(&self, order: &Order) -> bool {
        matches!(
            order.status,
            OrderStatus::Processing | OrderStatus::Completed | OrderStatus::OnHold
        )
    }

    /// Get total amount already refunded
    pub fn get_total_refunded(&self, _order: &Order) -> Decimal {
        // In full implementation, sum from refunds table
        Decimal::ZERO
    }

    /// Check if order is paid
    pub fn is_paid(&self, order: &Order) -> bool {
        order.paid_at.is_some()
    }

    /// Check if order is editable
    pub fn is_editable(&self, order: &Order) -> bool {
        matches!(
            order.status,
            OrderStatus::Pending | OrderStatus::OnHold | OrderStatus::Draft
        )
    }

    /// Calculate item totals
    pub fn calculate_item_totals(&self, item: &mut OrderItem) {
        item.subtotal = item.unit_price * Decimal::from(item.quantity);
        item.total = item.subtotal;
    }

    /// Recalculate order totals
    pub fn recalculate_totals(&self, order: &mut Order) {
        let mut subtotal = Decimal::ZERO;
        let mut item_tax = Decimal::ZERO;

        for item in &order.items {
            subtotal += item.subtotal;
            item_tax += item.subtotal_tax;
        }

        let shipping_total: Decimal = order.shipping_lines.iter()
            .map(|s| s.total)
            .sum();
        let shipping_tax: Decimal = order.shipping_lines.iter()
            .map(|s| s.total_tax)
            .sum();

        let fee_total: Decimal = order.fee_lines.iter()
            .map(|f| f.total)
            .sum();
        let fee_tax: Decimal = order.fee_lines.iter()
            .map(|f| f.total_tax)
            .sum();

        let discount_total: Decimal = order.coupon_lines.iter()
            .map(|c| c.discount)
            .sum();
        let discount_tax: Decimal = order.coupon_lines.iter()
            .map(|c| c.discount_tax)
            .sum();

        order.subtotal = subtotal;
        order.cart_tax = item_tax;
        order.shipping_total = shipping_total;
        order.shipping_tax = shipping_tax;
        order.discount_total = discount_total;
        order.discount_tax = discount_tax;
        order.total_tax = item_tax + shipping_tax + fee_tax - discount_tax;
        order.total = subtotal + shipping_total + fee_total + order.total_tax - discount_total;
        order.updated_at = Some(Utc::now());
    }

    /// Add item to order
    pub fn add_item(&self, order: &mut Order, item: OrderItem) -> Result<(), OrderError> {
        if !self.is_editable(order) {
            return Err(OrderError::OrderLocked);
        }

        order.items.push(item);
        self.recalculate_totals(order);
        Ok(())
    }

    /// Remove item from order
    pub fn remove_item(&self, order: &mut Order, item_id: Uuid) -> Result<OrderItem, OrderError> {
        if !self.is_editable(order) {
            return Err(OrderError::OrderLocked);
        }

        let index = order.items.iter().position(|i| i.id == item_id)
            .ok_or(OrderError::NotFound)?;

        let item = order.items.remove(index);
        self.recalculate_totals(order);
        Ok(item)
    }

    /// Update item quantity
    pub fn update_item_quantity(
        &self,
        order: &mut Order,
        item_id: Uuid,
        quantity: i32,
    ) -> Result<(), OrderError> {
        if !self.is_editable(order) {
            return Err(OrderError::OrderLocked);
        }

        let item = order.items.iter_mut()
            .find(|i| i.id == item_id)
            .ok_or(OrderError::NotFound)?;

        item.quantity = quantity;
        item.subtotal = item.unit_price * Decimal::from(quantity);
        item.total = item.subtotal;

        self.recalculate_totals(order);
        Ok(())
    }

    /// Add shipping line
    pub fn add_shipping_line(&self, order: &mut Order, shipping: OrderShippingLine) -> Result<(), OrderError> {
        if !self.is_editable(order) {
            return Err(OrderError::OrderLocked);
        }

        order.shipping_lines.push(shipping);
        self.recalculate_totals(order);
        Ok(())
    }

    /// Add fee line
    pub fn add_fee_line(&self, order: &mut Order, fee: OrderFeeLine) -> Result<(), OrderError> {
        if !self.is_editable(order) {
            return Err(OrderError::OrderLocked);
        }

        order.fee_lines.push(fee);
        self.recalculate_totals(order);
        Ok(())
    }

    /// Get order status label
    pub fn get_status_label(&self, status: OrderStatus) -> &'static str {
        match status {
            OrderStatus::Pending => "Pending payment",
            OrderStatus::Processing => "Processing",
            OrderStatus::OnHold => "On hold",
            OrderStatus::Completed => "Completed",
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::Refunded => "Refunded",
            OrderStatus::Failed => "Failed",
            OrderStatus::Draft => "Draft",
            OrderStatus::Checkout => "Checkout",
        }
    }

    /// Get orders needing processing
    pub fn get_processing_count(&self, orders: &[Order]) -> usize {
        orders.iter()
            .filter(|o| o.status == OrderStatus::Processing)
            .count()
    }

    /// Get orders on hold
    pub fn get_on_hold_count(&self, orders: &[Order]) -> usize {
        orders.iter()
            .filter(|o| o.status == OrderStatus::OnHold)
            .count()
    }

    /// Format order for display
    pub fn format_order_summary(&self, order: &Order) -> OrderSummary {
        OrderSummary {
            id: order.id,
            order_number: order.order_number.clone(),
            status: order.status,
            status_label: self.get_status_label(order.status).to_string(),
            customer_name: format!("{} {}", order.billing_first_name, order.billing_last_name),
            customer_email: order.billing_email.clone(),
            total: order.total,
            currency: order.currency.clone(),
            item_count: order.items.iter().map(|i| i.quantity).sum(),
            created_at: order.created_at,
        }
    }
}

/// Order summary for lists
#[derive(Debug, Clone)]
pub struct OrderSummary {
    pub id: Uuid,
    pub order_number: String,
    pub status: OrderStatus,
    pub status_label: String,
    pub customer_name: String,
    pub customer_email: String,
    pub total: Decimal,
    pub currency: String,
    pub item_count: i32,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let settings = RustCommerceSettings::default();
        let service = OrderService::new(settings);

        // Valid transitions
        assert!(service.can_transition(OrderStatus::Pending, OrderStatus::Processing));
        assert!(service.can_transition(OrderStatus::Processing, OrderStatus::Completed));
        assert!(service.can_transition(OrderStatus::Completed, OrderStatus::Refunded));

        // Invalid transitions
        assert!(!service.can_transition(OrderStatus::Completed, OrderStatus::Pending));
        assert!(!service.can_transition(OrderStatus::Refunded, OrderStatus::Processing));
    }

    #[test]
    fn test_status_labels() {
        let settings = RustCommerceSettings::default();
        let service = OrderService::new(settings);

        assert_eq!(service.get_status_label(OrderStatus::Pending), "Pending payment");
        assert_eq!(service.get_status_label(OrderStatus::Processing), "Processing");
        assert_eq!(service.get_status_label(OrderStatus::Completed), "Completed");
    }
}
