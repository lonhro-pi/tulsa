mod db;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Database URL
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./creator_crm.db?mode=rwc".to_string());

    // Initialize database
    let pool = db::init_db(&database_url)
        .await
        .expect("Failed to initialize database");

    let app_state = Arc::new(pool);

    // Build router
    let api_routes = Router::new()
        // Subscriber routes
        .route("/api/subscribers", post(handlers::create_subscriber_handler))
        .route("/api/subscribers", get(handlers::list_subscribers_handler))
        .route("/api/subscribers/:id", get(handlers::get_subscriber_handler))
        .route("/api/subscribers/:id", put(handlers::update_subscriber_handler))
        .route("/api/subscribers/:id", delete(handlers::delete_subscriber_handler))
        // Interaction routes
        .route("/api/interactions", post(handlers::create_interaction_handler))
        .route("/api/subscribers/:id/interactions", get(handlers::get_subscriber_interactions_handler))
        // Stats routes
        .route("/api/stats", get(handlers::get_stats_handler))
        .route("/api/expiring", get(handlers::get_expiring_subscriptions_handler))
        .with_state(app_state);

    // Serve static files
    let static_service = ServeDir::new("static");

    let app = Router::new()
        .nest_service("/", static_service)
        .merge(api_routes)
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()));

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port");

    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
