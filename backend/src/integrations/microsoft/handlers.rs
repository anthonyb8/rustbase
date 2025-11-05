use crate::{data::Token, error::Result, state::AppState};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use oauth2::AuthorizationCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    state: String,
    code: AuthorizationCode,
}

pub async fn outlook_messages(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(user_id, "microsoft")
        .await?;

    let messages = state
        .integration
        .microsoft
        .get_outlook_messages(&token.access_token, 10)
        .await?;

    println!("{:?}", messages);

    Ok(())
}
