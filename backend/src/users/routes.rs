use crate::middleware::auth_middleware;
use crate::state::AppState;
use crate::users::authentication::routes::router as auth_router;
use crate::users::handlers::{create_user, get_user};
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};
use std::sync::Arc;

// pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new().nest("/authenticate", auth_router());

    let protected_routes = Router::new()
        .route("/get/{id}", get(get_user))
        .route("/create", post(create_user))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
