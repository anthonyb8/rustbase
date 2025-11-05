use super::handlers::get_authentication_url;
use super::handlers::notion_callback;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/oauth/authenticate/{id}", get(get_authentication_url))
        .route("/oauth/callback", get(notion_callback))
}
