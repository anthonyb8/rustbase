use crate::examples::handlers::{list_files, upload_file};
use crate::middleware::auth_middleware;
use crate::state::AppState;
use axum::routing::{delete, get, patch, post, put};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/upload/file/{id}", post(upload_file))
        .route("/list/file/{id}", get(list_files));

    let protected_routes = Router::new()
        .route("/other/{id}", post(upload_file))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
