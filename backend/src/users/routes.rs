use crate::middleware::auth_middleware;
use crate::state::AppState;
use crate::users::handlers::{delete_user, get_user, update_user_email, update_user_password};
use axum::routing::{delete, get, put};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user))
        .route("/", delete(delete_user))
        .route("/email", put(update_user_email))
        .route("/password", put(update_user_password))
        // .route("/", put(delete_user))
        .route_layer(middleware::from_fn(auth_middleware))
}
