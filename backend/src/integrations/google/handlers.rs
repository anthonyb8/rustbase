use super::data::MessageQuery;
use crate::crypt::jwt::Claims;
use crate::data::{Event, Token};
use crate::error::Result;
use crate::response::ApiResponse;
use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{self, Extension};
use serde_json::json;
use std::sync::Arc;

// Gmail
pub async fn gmail_messages(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    // let user_id = Uuid::parse_str(&claims.sub)?;

    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(&claims.sub, "google")
        .await?;

    let messages = state
        .integration
        .google
        .get_gmail_messages(&token.access_token, 10)
        .await?;

    Ok(ApiResponse::new(
        StatusCode::OK,
        "Password updated successfully",
        messages,
    ))
}

pub async fn gmail_message(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<MessageQuery>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(&claims.sub, "google")
        .await?;

    let message = state
        .integration
        .google
        .get_message_details(&token.access_token, &query.id)
        .await?;

    Ok(ApiResponse::new(
        StatusCode::OK,
        "Password updated successfully",
        message,
    ))
}

pub async fn gmail_subscription(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    Ok(())
}

pub async fn gmail_callback(State(state): State<Arc<AppState>>, Path(id): Path<String>) {
    let key = format!("ws:{}", id);

    let event = Event {
        name: "event".to_string(),
        data: json!({"key": "value"}),
    };

    state
        .storage
        .redis
        .append_event_queue(&key, &event)
        .await
        .expect("errror");
}
