//! Memberships API Handlers
//!
//! HTTP request handlers for membership management.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct MembershipQuery {
    pub status: Option<String>,
    pub plan_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMembershipRequest {
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub start_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContentAccessRequest {
    pub content_id: Uuid,
    pub content_type: String,
}

#[derive(Debug, Serialize)]
pub struct ContentAccessResponse {
    pub has_access: bool,
    pub required_plans: Vec<Uuid>,
    pub upgrade_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MembershipResponse {
    pub id: Uuid,
    pub status: String,
    pub plan_id: Uuid,
    pub customer_id: Uuid,
    pub start_date: String,
    pub end_date: Option<String>,
    pub benefits: Vec<String>,
}
