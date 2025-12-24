//! Invoices API Handlers
//!
//! HTTP request handlers for invoice management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InvoiceQuery {
    pub status: Option<String>,
    pub customer_id: Option<Uuid>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub overdue: Option<bool>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceRequest {
    pub customer_id: Uuid,
    pub order_id: Option<Uuid>,
    pub invoice_type: Option<String>,
    pub due_date: Option<String>,
    pub items: Vec<InvoiceItemRequest>,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceItemRequest {
    pub item_type: Option<String>,
    pub product_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: String,
    pub unit_price: String,
    pub discount: Option<String>,
    pub tax_rate: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordPaymentRequest {
    pub amount: String,
    pub payment_method: String,
    pub payment_date: String,
    pub transaction_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvoiceResponse {
    pub id: Uuid,
    pub invoice_number: String,
    pub invoice_type: String,
    pub status: String,
    pub customer_id: Uuid,
    pub customer_name: String,
    pub subtotal: String,
    pub tax_total: String,
    pub total: String,
    pub amount_paid: String,
    pub amount_due: String,
    pub invoice_date: String,
    pub due_date: Option<String>,
    pub is_overdue: bool,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvoiceDetailResponse {
    pub invoice: InvoiceResponse,
    pub seller: PartyInfo,
    pub buyer: PartyInfo,
    pub items: Vec<InvoiceItemResponse>,
    pub payments: Vec<PaymentResponse>,
}

#[derive(Debug, Serialize)]
pub struct PartyInfo {
    pub name: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub address: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvoiceItemResponse {
    pub name: String,
    pub description: Option<String>,
    pub quantity: String,
    pub unit_price: String,
    pub discount: String,
    pub tax_rate: String,
    pub total: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub amount: String,
    pub payment_method: String,
    pub payment_date: String,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvoiceStatsResponse {
    pub total_invoiced: String,
    pub total_paid: String,
    pub total_outstanding: String,
    pub total_overdue: String,
    pub invoice_count: i64,
    pub overdue_count: i64,
}
