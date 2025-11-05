use crate::{data::Token, error::Result, state::AppState};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn get_workspace_info(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(user_id, "notion")
        .await?;

    let messages = state
        .integration
        .notion
        .get_workspace_info(&token.access_token)
        .await?;

    println!("{:?}", messages);

    Ok(())
}
