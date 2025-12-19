use axum::{routing::get, Router};

use crate::state;
use crate::state::app_state::AppState;
use crate::router::health::health_check;

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/", get(root))
    .route("/health", get(crate::router::health::health_check))
    .with_state(state)
}

pub async fn root() -> &'static str {
    "Rust backend is running!"
}