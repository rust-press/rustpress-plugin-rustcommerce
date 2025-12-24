//! Bookings API Handlers
//!
//! HTTP request handlers for booking management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BookingQuery {
    pub status: Option<String>,
    pub resource_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub product_id: Uuid,
    pub resource_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub start_date: String,
    pub start_time: Option<String>,
    pub end_date: Option<String>,
    pub end_time: Option<String>,
    pub persons: i32,
}

#[derive(Debug, Deserialize)]
pub struct AvailabilityQuery {
    pub product_id: Uuid,
    pub resource_id: Option<Uuid>,
    pub date: String,
    pub duration: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct AvailabilitySlot {
    pub start_time: String,
    pub end_time: String,
    pub available: bool,
    pub remaining_capacity: i32,
}

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: Uuid,
    pub status: String,
    pub product_id: Uuid,
    pub customer_id: Uuid,
    pub start_datetime: String,
    pub end_datetime: String,
    pub total: String,
}
