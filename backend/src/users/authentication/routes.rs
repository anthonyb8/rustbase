use super::handlers::{
    login_user, logout, refresh, register_user, reset_password, send_mfa, send_reset_password,
    verification_email, verify_email, verify_mfa,
};
use crate::middleware::{auth_middleware, auth_partial_middleware};
use crate::state::AppState;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/refresh", get(refresh))
        .route("/send-verify-email", post(verification_email))
        .route("/verify-email", post(verify_email))
        .route("/reset-password", post(reset_password))
        .route("/send-reset-password", post(send_reset_password));

    let partially_protected = Router::new()
        .route("/verify-mfa", post(verify_mfa))
        .route("/send-mfa", post(send_mfa))
        .route_layer(middleware::from_fn(auth_partial_middleware));

    let protected_routes = Router::new()
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(partially_protected)
}
