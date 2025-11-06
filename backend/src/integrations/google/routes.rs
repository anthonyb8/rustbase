use super::handlers::{gmail_message, gmail_messages};
use crate::integrations::google::handlers::{gmail_callback, gmail_subscription};
use crate::middleware::auth_middleware;
use crate::state::AppState;
use axum::routing::get;
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new().route("/gmail/callback/{id}", get(gmail_callback));

    let protected_routes = Router::new()
        .route("/gmail/messages", get(gmail_messages))
        .route("/gmail/message", get(gmail_message))
        .route("/gmail/subcription", get(gmail_subscription))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
