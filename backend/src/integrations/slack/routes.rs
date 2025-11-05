use crate::integrations::slack::handlers::get_user_info;
use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/outlook/messages/{id}", get(get_user_info))
        .route("/gmail/message/{id}", get(get_user_info))
}
