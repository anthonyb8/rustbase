use crate::middleware::auth_partial_middleware;
use crate::smtp::handlers::{mfa_email, reset_password_email, verification_email};
use crate::state::AppState;
use axum::routing::post;
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/verification-email", post(verification_email))
        .route("/forgot-password", post(reset_password_email));

    let partially_protected = Router::new()
        .route("/mfa-code", post(mfa_email))
        .route_layer(middleware::from_fn(auth_partial_middleware));

    Router::new()
        .merge(public_routes)
        .merge(partially_protected)
}
