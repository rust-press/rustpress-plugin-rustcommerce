//! Booking Models
//!
//! Appointment and reservation booking system.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime, Weekday};
use std::collections::HashMap;

/// Bookable product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookableProduct {
    pub id: Uuid,
    pub product_id: Uuid,
    pub booking_type: BookingType,
    pub duration: i32,
    pub duration_unit: DurationUnit,
    pub min_duration: Option<i32>,
    pub max_duration: Option<i32>,
    pub buffer_before: Option<i32>,
    pub buffer_after: Option<i32>,
    pub calendar_display: CalendarDisplay,
    pub requires_confirmation: bool,
    pub can_be_cancelled: bool,
    pub cancel_limit: Option<i32>, // Hours before start
    pub has_persons: bool,
    pub min_persons: Option<i32>,
    pub max_persons: Option<i32>,
    pub has_resources: bool,
    pub resources_assignment: ResourceAssignment,
    pub default_date_availability: DefaultAvailability,
    pub check_availability: bool,
    pub availability_rules: Vec<AvailabilityRule>,
    pub pricing_rules: Vec<PricingRule>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookingType {
    FixedTime,
    FixedDuration,
    CustomerDefined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnit {
    Minute,
    Hour,
    Day,
    Month,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarDisplay {
    Always,
    DateRange,
    MonthsAhead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceAssignment {
    CustomerSelect,
    AutoAssign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultAvailability {
    Available,
    NotAvailable,
}

/// Booking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub product_id: Uuid,
    pub order_id: Option<Uuid>,
    pub order_item_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub status: BookingStatus,

    // Timing
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub all_day: bool,

    // Details
    pub persons: i32,
    pub resource_id: Option<Uuid>,
    pub cost: Decimal,

    // Customer info
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_note: Option<String>,

    // Metadata
    pub meta: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookingStatus {
    Unpaid,
    Pending,
    Confirmed,
    Paid,
    Complete,
    InCart,
    Cancelled,
    WasPInCart,
}

/// Booking resource (staff member, room, equipment, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingResource {
    pub id: Uuid,
    pub site_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub base_cost: Option<Decimal>,
    pub block_cost: Option<Decimal>,
    pub availability_rules: Vec<AvailabilityRule>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

/// Availability rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRule {
    pub id: Uuid,
    pub rule_type: AvailabilityRuleType,
    pub priority: i32,
    pub bookable: bool,

    // Time range
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub from_time: Option<NaiveTime>,
    pub to_time: Option<NaiveTime>,

    // Day of week
    pub days_of_week: Option<Vec<Weekday>>,

    // Specific dates
    pub specific_dates: Option<Vec<NaiveDate>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityRuleType {
    Custom,
    DateRange,
    TimeRange,
    DayOfWeek,
    SpecificDate,
    Holiday,
}

/// Pricing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub id: Uuid,
    pub rule_type: PricingRuleType,
    pub priority: i32,
    pub modifier: PriceModifier,
    pub amount: Decimal,

    // Conditions
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub from_time: Option<NaiveTime>,
    pub to_time: Option<NaiveTime>,
    pub days_of_week: Option<Vec<Weekday>>,
    pub min_persons: Option<i32>,
    pub max_persons: Option<i32>,
    pub min_duration: Option<i32>,
    pub max_duration: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PricingRuleType {
    DateRange,
    TimeRange,
    DayOfWeek,
    PersonCount,
    Duration,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceModifier {
    Add,
    Subtract,
    Multiply,
    Divide,
    Replace,
}

/// Time slot for availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub available: bool,
    pub remaining_capacity: i32,
    pub price: Decimal,
}

/// Booking availability check request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityCheckRequest {
    pub product_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub persons: Option<i32>,
    pub resource_id: Option<Uuid>,
}

/// Booking availability check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityCheckResponse {
    pub available_slots: Vec<TimeSlot>,
    pub unavailable_dates: Vec<NaiveDate>,
    pub fully_booked_dates: Vec<NaiveDate>,
}

impl Booking {
    /// Get duration in minutes
    pub fn duration_minutes(&self) -> i64 {
        (self.end_date - self.start_date).num_minutes()
    }

    /// Check if booking is in the past
    pub fn is_past(&self) -> bool {
        self.end_date < Utc::now()
    }

    /// Check if booking is ongoing
    pub fn is_ongoing(&self) -> bool {
        let now = Utc::now();
        self.start_date <= now && self.end_date >= now
    }

    /// Check if can be cancelled
    pub fn can_cancel(&self) -> bool {
        matches!(
            self.status,
            BookingStatus::Pending | BookingStatus::Confirmed | BookingStatus::Paid
        ) && !self.is_past()
    }
}
