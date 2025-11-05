use crate::integrations::microsoft::handlers::outlook_messages;
use crate::middleware::auth_middleware;
use crate::state::AppState;
use axum::routing::get;
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/outlook/messages", get(outlook_messages))
        .route_layer(middleware::from_fn(auth_middleware))
}
