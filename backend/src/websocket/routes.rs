use super::handlers::handler;
use crate::middleware::auth_middleware;
use crate::state::AppState;
use axum::routing::any;
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/initialize/{id}", any(handler))
    // .route_layer(middleware::from_fn(auth_middleware))
}
