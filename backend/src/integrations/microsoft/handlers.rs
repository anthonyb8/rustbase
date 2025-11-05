use crate::{crypt::jwt::Claims, data::Token, error::Result, state::AppState};
use axum::{extract::State, response::IntoResponse, Extension};
use std::sync::Arc;

pub async fn outlook_messages(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let token: Token = state
        .storage
        .redis
        .get_oauth_tokens(&claims.sub, "microsoft")
        .await?;

    let messages = state
        .integration
        .microsoft
        .get_outlook_messages(&token.access_token, 10)
        .await?;

    println!("{:?}", messages);

    Ok(())
}
