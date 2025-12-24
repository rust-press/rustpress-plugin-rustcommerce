//! Tax API Handlers
//!
//! REST API endpoints for tax management.

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::tax::{TaxRate, TaxRateFilter, CreateTaxRateRequest, UpdateTaxRateRequest};

/// List tax rates
/// GET /rc/v1/taxes
pub async fn list_tax_rates(
    Query(filter): Query<TaxRateFilter>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "rates": [],
            "total": 0,
            "page": filter.page.unwrap_or(1),
            "per_page": filter.per_page.unwrap_or(10)
        })),
    )
}

/// Get a single tax rate
/// GET /rc/v1/taxes/:id
pub async fn get_tax_rate(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "code": "tax_rate_not_found",
            "message": "Tax rate not found"
        })),
    )
}

/// Create a new tax rate
/// POST /rc/v1/taxes
pub async fn create_tax_rate(
    Json(request): Json<CreateTaxRateRequest>,
) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": Uuid::now_v7(),
            "name": request.name,
            "rate": request.rate.to_string(),
            "message": "Tax rate created successfully"
        })),
    )
}

/// Update a tax rate
/// PUT /rc/v1/taxes/:id
pub async fn update_tax_rate(
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTaxRateRequest>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "id": id,
            "message": "Tax rate updated successfully"
        })),
    )
}

/// Delete a tax rate
/// DELETE /rc/v1/taxes/:id
pub async fn delete_tax_rate(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

/// Batch update tax rates
/// POST /rc/v1/taxes/batch
pub async fn batch_update_tax_rates(
    Json(request): Json<BatchTaxRateRequest>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "created": [],
            "updated": [],
            "deleted": []
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct BatchTaxRateRequest {
    pub create: Option<Vec<CreateTaxRateRequest>>,
    pub update: Option<Vec<UpdateTaxRateRequest>>,
    pub delete: Option<Vec<Uuid>>,
}

/// List tax classes
/// GET /rc/v1/taxes/classes
pub async fn list_tax_classes() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "classes": [
                { "slug": "standard", "name": "Standard rate" },
                { "slug": "reduced-rate", "name": "Reduced rate" },
                { "slug": "zero-rate", "name": "Zero rate" }
            ]
        })),
    )
}

/// Create a tax class
/// POST /rc/v1/taxes/classes
pub async fn create_tax_class(
    Json(request): Json<CreateTaxClassRequest>,
) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "slug": request.slug.unwrap_or_else(|| slugify(&request.name)),
            "name": request.name,
            "message": "Tax class created"
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct CreateTaxClassRequest {
    pub name: String,
    pub slug: Option<String>,
}

/// Delete a tax class
/// DELETE /rc/v1/taxes/classes/:slug
pub async fn delete_tax_class(
    Path(slug): Path<String>,
) -> impl IntoResponse {
    if slug == "standard" {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "code": "cannot_delete",
                "message": "Cannot delete the standard tax class"
            })),
        );
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Tax class deleted"
        })),
    )
}

/// Calculate tax for location
/// POST /rc/v1/taxes/calculate
pub async fn calculate_tax(
    Json(request): Json<CalculateTaxRequest>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "tax_amount": "0.00",
            "tax_rate": "0",
            "tax_lines": []
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct CalculateTaxRequest {
    pub country: String,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub city: Option<String>,
    pub tax_class: Option<String>,
    pub amount: String,
    pub prices_include_tax: Option<bool>,
}

/// Simple slug function
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
