use crate::middleware::auth_middleware;
use crate::objects::handlers::{delete_files, get_file, list_files, upload_file};
use crate::state::AppState;
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/upload/file/{id}", post(upload_file))
        .route("/upload/file/{id}", get(get_file))
        .route("/list/file/{id}", get(list_files))
        .route("/delete/file/{id}", delete(delete_files));

    let protected_routes = Router::new()
        .route("/other/{id}", post(upload_file))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
