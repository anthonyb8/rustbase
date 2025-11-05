use crate::{crypt::jwt::Claims, data::Token, error::Result, state::AppState};
use axum::{extract::State, response::IntoResponse, Extension};
use std::sync::Arc;

pub async fn get_workspace_info(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(&claims.sub, "notion")
        .await?;

    let messages = state
        .integration
        .notion
        .get_workspace_info(&token.access_token)
        .await?;

    println!("{:?}", messages);

    Ok(())
}
