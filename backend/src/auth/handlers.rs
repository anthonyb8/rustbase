use super::models::{Code, VerifyQuery};
use super::queries::{AuthQueries, TokenQueries};
use crate::auth::models::{LoginUser, NewPassword, RegisterUser};
use crate::config::CONFIG;
use crate::crypt::hash::verify_password;
use crate::crypt::jwt::{encode_jwt, Claims};
use crate::crypt::tokens::generate_token;
use crate::error::Result;
use crate::response::ApiResponse;
use crate::state::AppState;
use crate::Error;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::{Duration, Utc};
use cookie::time;
use serde_json::json;
use sqlx::types::Uuid;
use std::sync::Arc;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(user): Json<RegisterUser>,
) -> Result<impl IntoResponse> {
    let mut tx = state.storage.postgres.start_transaction().await?;

    match AuthQueries::register(&user, &mut tx).await {
        Ok(id) => {
            tx.commit().await?;
            Ok(ApiResponse::new(
                StatusCode::CREATED,
                &format!("Successfully created user with id {}", id),
                "".to_string(),
            ))
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e.into())
        }
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(user): Json<LoginUser>,
) -> Result<impl IntoResponse> {
    let (id, hash) = AuthQueries::get_password(&user.email, &state.storage.postgres.pool)
        .await
        .map_err(|_| Error::CustomError("Invalid email or password".into()))?;

    if !verify_password(&user.password, &hash)? {
        return Err(Error::CustomError("Invalid email or password.".into()));
    }

    let jwt = encode_jwt(id.to_string(), false)?;

    Ok(ApiResponse::new(
        StatusCode::OK,
        &format!("User successfully logged in"),
        json!({"token" :jwt, "token_type": "Bearer"}),
    ))
}

pub async fn verify_email(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VerifyQuery>,
) -> Result<impl IntoResponse> {
    let user_id = match state
        .storage
        .redis
        .get_token(&query.token, "verify-email")
        .await
    {
        Ok(user_id) => user_id,
        Err(_) => return Err(Error::CustomError("Invalid or expired token".into())),
    };

    AuthQueries::verify_user(user_id, &state.storage.postgres.pool).await?;

    Ok(ApiResponse::new(
        StatusCode::CREATED,
        &format!("Login."),
        "",
    ))
}

pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VerifyQuery>,
    Json(body): Json<NewPassword>,
) -> Result<impl IntoResponse> {
    let user_id = match state
        .storage
        .redis
        .get_token(&query.token, "reset-password")
        .await
    {
        Ok(user_id) => user_id,
        Err(_) => return Err(Error::CustomError("Invalid or expired token".into())),
    };

    let mut tx = state.storage.postgres.start_transaction().await?;

    match AuthQueries::update_password(user_id, &body.password, &mut tx).await {
        Ok(_) => {
            tx.commit().await?;
            Ok(ApiResponse::new(
                StatusCode::OK,
                "Successfully updated password.",
                "".to_string(),
            ))
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e.into())
        }
    }
}

pub async fn verify_mfa(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    jar: CookieJar,
    Json(code): Json<Code>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)?;

    let stored_code = match state.storage.redis.get_mfa_code(user_id).await? {
        Some(code) => code,
        None => return Err(Error::CustomError("MFA code invalid or expired".into())),
    };

    //  Compare codes
    if stored_code != code.code {
        return Err(Error::CustomError("Incorrect MFA code".into()));
    }

    // Create Refresh token
    let refresh_token = generate_token();
    let duration = Duration::days(CONFIG.refresh_token_expire_days.into());
    let expiry = Utc::now() + duration;
    TokenQueries::create_refresh_token(
        user_id,
        &refresh_token,
        expiry,
        &state.storage.postgres.pool,
    )
    .await?;

    let cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::days(
            CONFIG.refresh_token_expire_days.into(),
        ))
        .build();

    let jar = jar.add(cookie);

    // if not return jwt for full jwt
    let jwt = encode_jwt(claims.sub, true)?;

    Ok((
        jar,
        ApiResponse::new(
            StatusCode::OK,
            "User successfully verified.",
            json!({"token" :jwt, "token_type": "Bearer"}),
        ),
    ))
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    let cookie = jar
        .get("refresh_token")
        .ok_or_else(|| Error::CustomError("Refresh token not found.".into()))?;

    let token = cookie.value().to_owned();

    match TokenQueries::validate_refresh_token(&token, &state.storage.postgres.pool).await {
        Ok(user_id) => {
            // if not return jwt for full jwt
            let jwt = encode_jwt(user_id.to_string(), true)?;

            Ok((
                jar,
                ApiResponse::new(
                    StatusCode::OK,
                    "User successfully verified.",
                    json!({"token" :jwt, "token_type": "Bearer"}),
                ),
            ))
        }
        Err(e) => return Err(e),
    }
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    let cookie = jar
        .get("refresh_token")
        .ok_or_else(|| Error::CustomError("Refresh token not found.".into()))?;

    let token = cookie.value().to_owned();

    TokenQueries::delete_refresh_token(&token, &state.storage.postgres.pool).await?;

    let jar = jar.remove(Cookie::build("refresh_token").path("/").build());

    Ok((
        jar,
        ApiResponse::new(StatusCode::OK, &format!("Logout successuful."), ""),
    ))
}
