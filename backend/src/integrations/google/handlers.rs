use super::data::MessageQuery;
use crate::data::Token;
use crate::error::Result;
use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{self};
use serde::Deserialize;
use std::sync::Arc;

// Gmail
pub async fn gmail_messages(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(user_id, "google")
        .await?;

    let messages = state
        .integration
        .google
        .get_gmail_messages(&token.access_token, 10)
        .await?;

    println!("{:?}", messages);

    Ok(())
}

pub async fn gmail_message(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Query(query): Query<MessageQuery>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(user_id, "google")
        .await?;

    let message = state
        .integration
        .google
        .get_message_details(&token.access_token, &query.id)
        .await?;
    println!("{:?}", message);

    Ok(())
}
