//! Bookings Admin Module
//!
//! Admin handlers for booking management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Booking list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookingFilters {
    pub status: Option<String>,
    pub product_id: Option<Uuid>,
    pub resource_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Booking admin view data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingView {
    pub id: Uuid,
    pub booking_number: String,
    pub product_name: String,
    pub customer_name: String,
    pub status: String,
    pub start_date: String,
    pub end_date: String,
    pub persons: i32,
    pub cost: String,
    pub created_at: String,
}

/// Calendar view data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub title: String,
    pub start: String,
    pub end: String,
    pub color: String,
    pub url: String,
}

/// Resource admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceView {
    pub id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub bookings_count: i32,
}
