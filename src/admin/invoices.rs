//! Invoices Admin Module
//!
//! Admin handlers for invoice management.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Invoice list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceFilters {
    pub status: Option<String>,
    pub invoice_type: Option<String>,
    pub customer_id: Option<Uuid>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub overdue: Option<bool>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Invoice admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceView {
    pub id: Uuid,
    pub invoice_number: String,
    pub invoice_type: String,
    pub customer_name: String,
    pub customer_email: String,
    pub status: String,
    pub total: Decimal,
    pub amount_paid: Decimal,
    pub amount_due: Decimal,
    pub invoice_date: String,
    pub due_date: Option<String>,
    pub is_overdue: bool,
    pub days_overdue: Option<i64>,
}

/// Invoice detail view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceDetailView {
    pub id: Uuid,
    pub invoice_number: String,
    pub invoice_type: String,
    pub status: String,
    pub seller: PartyView,
    pub buyer: PartyView,
    pub items: Vec<InvoiceItemView>,
    pub subtotal: Decimal,
    pub discount_total: Decimal,
    pub tax_total: Decimal,
    pub shipping_total: Decimal,
    pub total: Decimal,
    pub amount_paid: Decimal,
    pub amount_due: Decimal,
    pub currency: String,
    pub invoice_date: String,
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub payment_method: Option<String>,
    pub notes: Option<String>,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyView {
    pub name: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub address: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItemView {
    pub name: String,
    pub description: Option<String>,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount: Decimal,
    pub tax_rate: Decimal,
    pub total: Decimal,
}

/// Invoice payment view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentView {
    pub id: Uuid,
    pub payment_date: String,
    pub amount: Decimal,
    pub payment_method: String,
    pub transaction_id: Option<String>,
    pub notes: Option<String>,
}

/// Invoice template view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateView {
    pub id: Uuid,
    pub name: String,
    pub invoice_type: String,
    pub is_default: bool,
    pub preview_url: Option<String>,
}

/// Invoice stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceStats {
    pub total_invoiced: Decimal,
    pub total_paid: Decimal,
    pub total_outstanding: Decimal,
    pub total_overdue: Decimal,
    pub invoice_count: i64,
    pub paid_count: i64,
    pub outstanding_count: i64,
    pub overdue_count: i64,
    pub avg_payment_days: f64,
}

/// Create invoice request for admin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceForm {
    pub invoice_type: String,
    pub customer_id: Uuid,
    pub order_id: Option<Uuid>,
    pub due_date: Option<String>,
    pub items: Vec<CreateItemForm>,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
    pub send_email: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemForm {
    pub item_type: String,
    pub product_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount: Option<Decimal>,
    pub tax_rate: Option<Decimal>,
}

/// Record payment form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPaymentForm {
    pub invoice_id: Uuid,
    pub amount: Decimal,
    pub payment_method: String,
    pub payment_date: String,
    pub transaction_id: Option<String>,
    pub notes: Option<String>,
    pub send_receipt: bool,
}
