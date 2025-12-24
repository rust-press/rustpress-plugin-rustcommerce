//! Invoice Management Models
//!
//! Self-contained invoice generation, PDF creation, and management.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use std::collections::HashMap;

/// Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub invoice_number: String,
    pub invoice_type: InvoiceType,
    pub status: InvoiceStatus,

    // References
    pub order_id: Option<Uuid>,
    pub subscription_id: Option<Uuid>,
    pub customer_id: Uuid,

    // Dates
    pub invoice_date: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub paid_date: Option<DateTime<Utc>>,

    // Parties
    pub seller: InvoiceParty,
    pub buyer: InvoiceParty,
    pub shipping_address: Option<InvoiceAddress>,

    // Items
    pub items: Vec<InvoiceItem>,

    // Totals
    pub subtotal: Decimal,
    pub discount_total: Decimal,
    pub tax_total: Decimal,
    pub shipping_total: Decimal,
    pub total: Decimal,
    pub amount_paid: Decimal,
    pub amount_due: Decimal,
    pub currency: String,

    // Tax details
    pub tax_breakdown: Vec<TaxBreakdown>,

    // Payment
    pub payment_method: Option<String>,
    pub payment_terms: Option<String>,
    pub payment_instructions: Option<String>,

    // Notes
    pub notes: Option<String>,
    pub internal_notes: Option<String>,
    pub terms_and_conditions: Option<String>,

    // PDF
    pub pdf_url: Option<String>,
    pub pdf_generated_at: Option<DateTime<Utc>>,

    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceType {
    Invoice,
    CreditNote,
    ProForma,
    Quote,
    Receipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Pending,
    Sent,
    Paid,
    PartiallyPaid,
    Overdue,
    Cancelled,
    Refunded,
    Void,
}

/// Invoice party (seller or buyer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceParty {
    pub name: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: InvoiceAddress,
    pub tax_id: Option<String>,
    pub vat_number: Option<String>,
    pub registration_number: Option<String>,
}

/// Invoice address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceAddress {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

/// Invoice line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub item_type: InvoiceItemType,
    pub product_id: Option<Uuid>,
    pub variation_id: Option<Uuid>,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: Decimal,
    pub unit: Option<String>,
    pub unit_price: Decimal,
    pub discount: Decimal,
    pub discount_percentage: Option<Decimal>,
    pub tax_rate: Decimal,
    pub tax_amount: Decimal,
    pub total: Decimal,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceItemType {
    Product,
    Service,
    Shipping,
    Fee,
    Discount,
    Tax,
    Custom,
}

/// Tax breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBreakdown {
    pub tax_name: String,
    pub tax_rate: Decimal,
    pub taxable_amount: Decimal,
    pub tax_amount: Decimal,
}

/// Invoice template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceTemplate {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub invoice_type: InvoiceType,
    pub template_html: String,
    pub template_css: Option<String>,
    pub is_default: bool,

    // Branding
    pub logo_url: Option<String>,
    pub header_color: Option<String>,
    pub accent_color: Option<String>,
    pub font_family: Option<String>,

    // Footer
    pub footer_text: Option<String>,
    pub show_payment_instructions: bool,
    pub show_terms: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Invoice number sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSequence {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub invoice_type: InvoiceType,
    pub prefix: String,
    pub suffix: Option<String>,
    pub next_number: i64,
    pub padding: i32,
    pub reset_yearly: bool,
    pub current_year: i32,
}

impl InvoiceSequence {
    /// Generate next invoice number
    pub fn next_invoice_number(&mut self) -> String {
        let number = format!(
            "{}{:0width$}{}",
            self.prefix,
            self.next_number,
            self.suffix.as_deref().unwrap_or(""),
            width = self.padding as usize
        );
        self.next_number += 1;
        number
    }
}

/// Invoice payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePayment {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub payment_date: DateTime<Utc>,
    pub amount: Decimal,
    pub payment_method: String,
    pub transaction_id: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Invoice email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceEmail {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub recipient_email: String,
    pub subject: String,
    pub body: String,
    pub status: EmailStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailStatus {
    Pending,
    Sent,
    Delivered,
    Opened,
    Failed,
    Bounced,
}

/// Invoice reminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceReminder {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub reminder_type: ReminderType,
    pub scheduled_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub status: EmailStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReminderType {
    DueSoon,
    Overdue,
    FinalNotice,
    Custom,
}

/// Invoice settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSettings {
    pub enabled: bool,
    pub auto_generate: bool,
    pub auto_send: bool,
    pub default_due_days: i32,
    pub default_payment_terms: String,
    pub default_notes: Option<String>,
    pub default_terms: Option<String>,
    pub send_reminders: bool,
    pub reminder_days_before: Vec<i32>,
    pub reminder_days_after: Vec<i32>,
    pub late_fee_enabled: bool,
    pub late_fee_type: LateFeeType,
    pub late_fee_amount: Option<Decimal>,
    pub late_fee_percentage: Option<Decimal>,
    pub logo_url: Option<String>,
    pub seller_info: Option<InvoiceParty>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LateFeeType {
    None,
    Fixed,
    Percentage,
    PerDay,
}

impl Default for InvoiceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_generate: true,
            auto_send: false,
            default_due_days: 30,
            default_payment_terms: "Net 30".to_string(),
            default_notes: None,
            default_terms: None,
            send_reminders: true,
            reminder_days_before: vec![7, 3, 1],
            reminder_days_after: vec![1, 7, 14],
            late_fee_enabled: false,
            late_fee_type: LateFeeType::None,
            late_fee_amount: None,
            late_fee_percentage: None,
            logo_url: None,
            seller_info: None,
        }
    }
}

impl Invoice {
    /// Check if invoice is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            self.amount_due > Decimal::ZERO && Utc::now() > due_date
        } else {
            false
        }
    }

    /// Calculate days overdue
    pub fn days_overdue(&self) -> Option<i64> {
        if let Some(due_date) = self.due_date {
            if self.is_overdue() {
                Some((Utc::now() - due_date).num_days())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if fully paid
    pub fn is_paid(&self) -> bool {
        self.amount_due <= Decimal::ZERO
    }

    /// Record payment
    pub fn record_payment(&mut self, amount: Decimal) {
        self.amount_paid += amount;
        self.amount_due = (self.total - self.amount_paid).max(Decimal::ZERO);

        if self.amount_due <= Decimal::ZERO {
            self.status = InvoiceStatus::Paid;
            self.paid_date = Some(Utc::now());
        } else {
            self.status = InvoiceStatus::PartiallyPaid;
        }

        self.updated_at = Some(Utc::now());
    }
}

/// Create invoice request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub invoice_type: InvoiceType,
    pub order_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub due_date: Option<NaiveDate>,
    pub items: Vec<CreateInvoiceItemRequest>,
    pub notes: Option<String>,
    pub payment_terms: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceItemRequest {
    pub item_type: InvoiceItemType,
    pub product_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount: Option<Decimal>,
    pub tax_rate: Option<Decimal>,
}

/// Send invoice request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInvoiceRequest {
    pub invoice_id: Uuid,
    pub recipient_email: String,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub attach_pdf: bool,
}

/// Invoice response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub invoice: Invoice,
    pub pdf_url: Option<String>,
}

/// Invoice analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    pub total_invoiced: Decimal,
    pub total_paid: Decimal,
    pub total_outstanding: Decimal,
    pub total_overdue: Decimal,

    pub invoice_count: i64,
    pub paid_count: i64,
    pub outstanding_count: i64,
    pub overdue_count: i64,

    pub avg_payment_days: f64,
    pub collection_rate: Decimal,
}
