//! Memberships Admin Module
//!
//! Admin handlers for membership management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Membership list filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MembershipFilters {
    pub status: Option<String>,
    pub plan_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Membership admin view data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipView {
    pub id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub plan_name: String,
    pub status: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub in_trial: bool,
}

/// Membership plan admin view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanView {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub member_count: i32,
    pub access_method: String,
}
