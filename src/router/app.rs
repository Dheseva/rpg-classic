use axum::{routing::{get,post}, Router};
use tower_http::trace::TraceLayer;

use crate::state::app_state::AppState;
use crate::router::{health::health_check, character::create_character};
// use crate::middleware::logging::logging_layer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/", get(root))
    .route("/health", get(health_check))
    .route("/characters/create", post(create_character))
    .with_state(state)
    .layer(TraceLayer::new_for_http())
}

pub async fn root() -> &'static str {
    "Rust backend is running!"
}