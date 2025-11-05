use super::google::routes::router as google_router;
use super::microsoft::routes::router as microsoft_router;
use super::notion::routes::router as notion_router;
use super::slack::routes::router as slack_router;
use crate::state::AppState;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/google", google_router())
        .nest("/notion", notion_router())
        .nest("/slack", slack_router())
        .nest("/microsoft", microsoft_router())
}
