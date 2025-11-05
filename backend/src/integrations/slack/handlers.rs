use crate::{data::Token, error::Result, state::AppState};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn get_user_info(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(user_id, "slack")
        .await?;

    let messages = state
        .integration
        .slack
        .get_user_info(&token.access_token, user_id)
        .await?;

    println!("{:?}", messages);

    Ok(())
}
