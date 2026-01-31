use crate::{db, models::*};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

pub type AppState = Arc<SqlitePool>;

// Query parameters for filtering
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    platform: Option<String>,
    tier: Option<String>,
}

// Subscriber endpoints
pub async fn create_subscriber_handler(
    State(pool): State<AppState>,
    Json(payload): Json<CreateSubscriber>,
) -> Result<Json<Subscriber>, AppError> {
    let subscriber = db::create_subscriber(&pool, payload).await?;
    Ok(Json(subscriber))
}

pub async fn get_subscriber_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Subscriber>, AppError> {
    let subscriber = db::get_subscriber(&pool, &id).await?;
    Ok(Json(subscriber))
}

pub async fn list_subscribers_handler(
    State(pool): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<Vec<Subscriber>>, AppError> {
    let subscribers = db::list_subscribers(&pool, params.platform, params.tier).await?;
    Ok(Json(subscribers))
}

pub async fn update_subscriber_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSubscriber>,
) -> Result<Json<Subscriber>, AppError> {
    let subscriber = db::update_subscriber(&pool, &id, payload).await?;
    Ok(Json(subscriber))
}

pub async fn delete_subscriber_handler(
    State(pool): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    db::delete_subscriber(&pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Interaction endpoints
pub async fn create_interaction_handler(
    State(pool): State<AppState>,
    Json(payload): Json<CreateInteraction>,
) -> Result<Json<Interaction>, AppError> {
    let interaction = db::create_interaction(&pool, payload).await?;
    Ok(Json(interaction))
}

pub async fn get_subscriber_interactions_handler(
    State(pool): State<AppState>,
    Path(subscriber_id): Path<String>,
) -> Result<Json<Vec<Interaction>>, AppError> {
    let interactions = db::get_subscriber_interactions(&pool, &subscriber_id).await?;
    Ok(Json(interactions))
}

// Stats endpoint
pub async fn get_stats_handler(
    State(pool): State<AppState>,
) -> Result<Json<SubscriberStats>, AppError> {
    let stats = db::get_stats(&pool).await?;
    Ok(Json(stats))
}

// Expiring subscriptions endpoint
#[derive(Debug, Deserialize)]
pub struct ExpiringQuery {
    #[serde(default = "default_days")]
    days: i64,
}

fn default_days() -> i64 {
    7
}

pub async fn get_expiring_subscriptions_handler(
    State(pool): State<AppState>,
    Query(params): Query<ExpiringQuery>,
) -> Result<Json<Vec<Subscriber>>, AppError> {
    let subscribers = db::get_expiring_subscriptions(&pool, params.days).await?;
    Ok(Json(subscribers))
}

// Error handling
pub struct AppError(sqlx::Error);

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self.0 {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
