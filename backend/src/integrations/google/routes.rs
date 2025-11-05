use super::handlers::{gmail_message, gmail_messages};
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/gmail/messages/{id}", get(gmail_messages))
        .route("/gmail/message/{id}", get(gmail_message))
}
