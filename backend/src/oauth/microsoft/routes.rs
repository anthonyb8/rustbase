use super::handlers::{get_authentication_url, microsoft_callback};
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/oauth/authenticate/{id}", get(get_authentication_url))
        .route("/oauth", get(microsoft_callback))
    // .route("/oauth/refresh", post(refresh_tokens))
}
