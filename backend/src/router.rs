use crate::auth::routes::router as auth_router;
use crate::integrations::routes::router as integrations_router;
use crate::objects::routes::router as objects_router;
use crate::response::ApiResponse;
use crate::smtp::routes::router as email_router;
use crate::state::AppState;
use crate::users::routes::router as user_router;
use crate::websocket::routes::router as ws_router;
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
        .nest("/email", email_router())
        .nest("/objects", objects_router())
        .nest("/integrations", integrations_router())
        .nest("/ws", ws_router())
        .route("/health", get(health_check))
        .with_state(arc_state)
}

async fn health_check() -> impl IntoResponse {
    ApiResponse::new(StatusCode::OK, "Status healthy.", "")
}
