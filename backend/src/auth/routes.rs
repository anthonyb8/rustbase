use super::handlers::{login, register};
use super::oauth::routes::router as oauth_router;
use crate::auth::handlers::{logout, refresh, reset_password, verify_email, verify_mfa};
use crate::middleware::{auth_middleware, auth_partial_middleware};
use crate::state::AppState;
use axum::routing::post;
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify-email", post(verify_email))
        .route("/reset-password", post(reset_password))
        .route("/refresh", post(refresh))
        .nest("/oauth", oauth_router());

    let partially_protected = Router::new()
        .route("/verify-mfa", post(verify_mfa))
        .route_layer(middleware::from_fn(auth_partial_middleware));

    let protected_routes = Router::new()
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(partially_protected)
        .merge(protected_routes)
}
