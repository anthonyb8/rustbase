use crate::{
    data::{Flow, Token},
    error::Result,
    response::ApiResponse,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use oauth2::AuthorizationCode;
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    state: String,
    code: AuthorizationCode,
}

pub async fn get_authentication_url(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let auth = state
        .services
        .microsoft
        .get_authorization_url(user_id)
        .await;

    let flow = Flow {
        csrf_state: auth.csrf_state,
        pkce_verifier: auth.pkce_verifier,
    };

    state
        .storage
        .redis
        .store_flow(user_id, "microsoft", &flow)
        .await?;

    Ok(axum::Json(auth.authorize_url))
}

pub async fn microsoft_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthRequest>,
) -> Result<impl IntoResponse> {
    // Split `id:csrf_token`
    let parts: Vec<&str> = params.state.split(':').collect();
    if parts.len() != 2 {
        return Ok(ApiResponse::new(
            "failure",
            &format!("Invalid state format."),
            StatusCode::BAD_REQUEST,
            "",
        ));
    }

    let (id, csrf_token) = (parts[0], parts[1]);
    let stored_verifier: Flow = state.storage.redis.get_flow(id, "microsoft").await?;

    if csrf_token != stored_verifier.csrf_state.secret() {
        return Ok(ApiResponse::new(
            "failure",
            &format!("Invalid state."),
            StatusCode::BAD_REQUEST,
            "",
        ));
    }

    //Get token
    let auth = state
        .services
        .microsoft
        .exchange_code(params.code, stored_verifier.pkce_verifier.unwrap())
        .await?;

    // Save tokens
    let refresh_token = match auth.1 {
        Some(token) => Some(token.secret().to_string()),
        None => None,
    };

    let token = Token {
        access_token: auth.0.secret().to_string(),
        refresh_token,
    };

    state
        .storage
        .redis
        .store_oauth_tokens(id, "microsoft", &token)
        .await?;
    state.storage.redis.delete_flow(id, "microsoft").await?;

    Ok(ApiResponse::new(
        "success",
        &format!("Verification confirmed."),
        StatusCode::CREATED,
        "",
    ))
}

//TODO: finish testing
// pub async fn refresh_tokens(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse> {
//     let auth = state.microsoft_client.get_authorization_url().await;
//
//     Ok(axum::Json(auth))
// }
