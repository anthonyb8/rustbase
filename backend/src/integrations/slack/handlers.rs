use crate::{crypt::jwt::Claims, data::Token, error::Result, state::AppState};
use axum::{extract::State, response::IntoResponse, Extension};
use std::sync::Arc;

pub async fn get_user_info(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(&claims.sub, "slack")
        .await?;

    let messages = state
        .integration
        .slack
        .get_user_info(&token.access_token, 1)
        .await?;

    println!("{:?}", messages);

    Ok(())
}
