use super::handlers::{get_authentication_url, gmail_message, gmail_messages, google_callback};
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/oauth/authenticate/{id}", get(get_authentication_url))
        .route("/oauth", get(google_callback))
        // .route("/oauth/revoke", delete(revoke_tokens))
        // .route("/oauth/refresh", post(refresh_tokens))
        .route("/gmail/messages/{id}", get(gmail_messages))
        .route("/gmail/message/{id}", get(gmail_message))
}
