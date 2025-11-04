use crate::oauth::google::routes::router as google_router;
use crate::oauth::microsoft::routes::router as microsoft_router;
use crate::oauth::slack::routes::router as slack_router;
use crate::response::ApiResponse;
use crate::state::AppState;
use crate::users::authentication::routes::router as auth_router;
use crate::users::routes::router as user_router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router(state: AppState) -> Router {
    let arc_state = Arc::new(state);

    Router::new()
        .nest("/users", user_router())
        .nest("/auth", auth_router())
        .nest("/google", google_router())
        .nest("/slack", slack_router())
        .nest("/microsoft", microsoft_router())
        .route("/health", get(health_check))
        .with_state(arc_state)
}

async fn health_check() -> impl IntoResponse {
    ApiResponse::new("success", "Status healthy.", StatusCode::OK, "")
}
