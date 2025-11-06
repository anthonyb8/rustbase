use crate::middleware::auth_middleware;
use crate::objects::handlers::{delete_files, get_file, list_files, upload_file};
use crate::state::AppState;
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(upload_file))
        .route("/", get(get_file))
        .route("/", delete(delete_files))
        .route("/list", get(list_files))
        .route_layer(middleware::from_fn(auth_middleware))
}
