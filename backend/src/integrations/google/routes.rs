use super::handlers::{gmail_message, gmail_messages};
use crate::middleware::auth_middleware;
use crate::state::AppState;
use axum::routing::get;
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/gmail/messages", get(gmail_messages))
        .route("/gmail/message", get(gmail_message))
        .route_layer(middleware::from_fn(auth_middleware))
}
