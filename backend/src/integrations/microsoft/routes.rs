use crate::integrations::microsoft::handlers::outlook_messages;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/outlook/messages/{id}", get(outlook_messages))
        .route("/gmail/message/{id}", get(outlook_messages))
}
