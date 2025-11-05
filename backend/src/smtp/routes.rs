use crate::middleware::auth_partial_middleware;
use crate::smtp::handlers::{send_mfa, send_reset_password, verification_email};
use crate::state::AppState;
use axum::routing::post;
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/verification-email", post(verification_email))
        .route("/forgot-password", post(send_reset_password));

    let partially_protected = Router::new()
        .route("/mfa-code", post(send_mfa))
        .route_layer(middleware::from_fn(auth_partial_middleware));

    Router::new()
        .merge(public_routes)
        .merge(partially_protected)
}
