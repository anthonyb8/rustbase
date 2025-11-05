use crate::auth::oauth::AuthRequest;
use crate::data::{Flow, Token};
use crate::state::AppState;
use crate::{error::Result, response::ApiResponse};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{self};
use std::sync::Arc;

pub async fn get_authentication_url(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let auth = state.oauth.google.get_authorization_url(user_id);

    let flow = Flow {
        csrf_state: auth.csrf_state,
        pkce_verifier: auth.pkce_verifier,
    };

    state
        .storage
        .redis
        .store_flow(user_id, "google", &flow)
        .await?;

    Ok(axum::Json(auth.authorize_url))
}

pub async fn google_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthRequest>,
) -> Result<impl IntoResponse> {
    println!("hellllllllll");
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
    let stored_verifier: Flow = state.storage.redis.get_flow(id, "google").await?;

    if csrf_token != stored_verifier.csrf_state.secret() {
        return Ok(ApiResponse::new(
            "failure",
            &format!("Invalid state."),
            StatusCode::BAD_REQUEST,
            "",
        ));
    }

    let auth = state
        .oauth
        .google
        .exchange_code(params.code, stored_verifier.pkce_verifier.unwrap())
        .await?;

    // Insert refresh and access token to db
    let token = Token {
        access_token: auth.0.secret().to_string(),
        refresh_token: Some(auth.1.secret().to_string()),
    };

    state
        .storage
        .redis
        .store_oauth_tokens(id, "google", &token)
        .await?;
    state.storage.redis.delete_flow(id, "google").await?;

    Ok(ApiResponse::new(
        "success",
        &format!("Verification confirmed."),
        StatusCode::CREATED,
        "",
    ))
}
