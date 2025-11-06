use crate::{
    auth::oauth::AuthRequest,
    data::{Flow, Token},
    error::Result,
    response::ApiResponse,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use sqlx::types::Uuid;
use std::sync::Arc;

pub async fn get_authentication_url(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let user_id: Uuid = Uuid::parse_str(&id)?;
    let auth = state.oauth.slack.get_authorization_url(user_id);

    let flow = Flow {
        csrf_state: auth.csrf_state,
        pkce_verifier: None,
    };

    state
        .storage
        .redis
        .store_flow(user_id, "slack", &flow)
        .await?;

    Ok(axum::Json(auth.authorize_url))
}

pub async fn slack_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthRequest>,
) -> Result<impl IntoResponse> {
    // Split `id:csrf_token`
    let parts: Vec<&str> = params.state.split(':').collect();
    if parts.len() != 2 {
        return Ok(ApiResponse::new(
            StatusCode::BAD_REQUEST,
            &format!("Invalid state format."),
            "",
        ));
    }

    let (id, csrf_token) = (parts[0], parts[1]);
    let stored_verifier: Flow = state.storage.redis.get_flow(id, "slack").await?;
    if csrf_token != stored_verifier.csrf_state.secret() {
        return Ok(ApiResponse::new(
            StatusCode::BAD_REQUEST,
            &format!("Invalid state."),
            "",
        ));
    }

    let auth = state.oauth.slack.exchange_code(params.code).await?;

    // Insert refresh and access token to db
    let token = Token {
        access_token: auth.0.secret().to_string(),
        refresh_token: None,
        // Some(auth.1.secret().to_string()),
    };

    state
        .storage
        .redis
        .store_oauth_tokens(id, "slack", &token)
        .await?;
    state.storage.redis.delete_flow(id, "slack").await?;

    Ok(ApiResponse::new(
        StatusCode::CREATED,
        &format!("Verification confirmed."),
        "",
    ))
}

// //TODO: finish testing
// pub async fn refresh_tokens(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse> {
//     let auth = state.slack_client.get_authorization_url().await;
//
//     Ok(axum::Json(auth))
// }
//
// pub async fn revoke_tokens(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse> {
//     let auth = state.slack_client.get_authorization_url().await;
//
//     Ok(axum::Json(auth))
// }
