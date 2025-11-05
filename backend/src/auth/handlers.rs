use super::models::{AuthUser, Code, VerificationEmail, VerifyQuery};
use super::queries::{MfaQueries, TokenQueries, UserQueries};
use crate::config::CONFIG;
use crate::crypt::hash::verify_password;
use crate::crypt::jwt::{encode_jwt, Claims};
use crate::crypt::tokens::{generate_code, generate_token};
use crate::error::Result;
use crate::response::ApiResponse;
use crate::smtp::messages::{mfa_code_body, verify_email_body, Email};
use crate::state::AppState;
use crate::Error;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::{Duration, Utc};
use cookie::time;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::Address;
use serde_json::json;
use std::sync::Arc;

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<AuthUser>,
) -> Result<impl IntoResponse> {
    // println!("hello");
    let id = state
        .storage
        .postgres
        .with_transaction(|tx| {
            Box::pin(async move {
                let id = UserQueries::register(&user, tx).await?;
                Ok(id)
            })
        })
        .await?;

    Ok(ApiResponse::new(
        "success",
        &format!("User {} registered.", id),
        StatusCode::CREATED,
        id,
    ))
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<AuthUser>,
) -> Result<impl IntoResponse> {
    let credentials = UserQueries::get_user_by_email(&user.email, &state.storage.postgres.pool)
        .await
        .map_err(|_| Error::CustomError("Invalid email or password".into()))?;

    let valid = verify_password(&user.password, &credentials.password_hash)?;

    if !valid {
        return Err(Error::CustomError("Invalid email or password.".into()));
    }

    let jwt = encode_jwt(credentials.id as usize, false)?;

    Ok(ApiResponse::new(
        "success",
        &format!("User successfully logged in"),
        StatusCode::OK,
        json!({"token" :jwt, "token_type": "Bearer"}),
    ))
}

pub async fn send_mfa(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    // generate mfa code
    let code = generate_code(6);
    let expiry = Utc::now() + Duration::minutes(CONFIG.email_mfa_expire_minutes.into());

    // Get email
    let email = UserQueries::get_email(claims.sub as i32, &state.storage.postgres.pool).await?;

    // send email
    let email = Email {
        recipient: Mailbox::new(Some("".to_owned()), email.parse::<Address>()?),
        sender: Mailbox::new(
            Some("Info".to_owned()),
            CONFIG.smtp_email.parse::<Address>()?,
        ),
        subject: String::from("MFA Verification Code"),
        header: ContentType::TEXT_HTML,
        body: mfa_code_body(code.as_ref())?,
    };

    // insert code to database
    state
        .storage
        .postgres
        .with_transaction(|tx| {
            Box::pin(async move {
                MfaQueries::create_mfa_code(claims.sub as i32, code.clone(), expiry, tx).await?;
                Ok(())
            })
        })
        .await?;

    state.smtp.send_email(email)?;

    Ok(ApiResponse::new(
        "success",
        &format!("MFA Code sent."),
        StatusCode::OK,
        "",
    ))
}
pub async fn verify_mfa(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    jar: CookieJar,
    Json(code): Json<Code>,
) -> Result<impl IntoResponse> {
    // pull the codes
    let code =
        MfaQueries::get_mfa_code(claims.sub as i32, &code.code, &state.storage.postgres.pool)
            .await?;

    // check if expired
    if code.expires_at < Utc::now() {
        return Err(Error::CustomError("Invalid or expired MFA code".into()));
    }

    // Create Refresh token
    let refresh_token = generate_token();
    let duration = Duration::days(CONFIG.refresh_token_expire_days.into());
    let expiry = Utc::now() + duration;
    TokenQueries::create_refresh_token(
        claims.sub as i32,
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
            "success",
            "User successfully verified.",
            StatusCode::OK,
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
    println!("{:?}", token);

    match TokenQueries::validate_refresh_token(&token, &state.storage.postgres.pool).await {
        Ok(user_id) => {
            // if not return jwt for full jwt
            let jwt = encode_jwt(user_id as usize, true)?;

            Ok((
                jar,
                ApiResponse::new(
                    "success",
                    "User successfully verified.",
                    StatusCode::OK,
                    json!({"token" :jwt, "token_type": "Bearer"}),
                ),
            ))
        }
        Err(e) => return Err(e),
    }
}

pub async fn verification_email(
    State(state): State<Arc<AppState>>,
    Json(body): Json<VerificationEmail>,
) -> Result<impl IntoResponse> {
    let token = generate_token();
    let expiry = Utc::now() + Duration::days(CONFIG.verify_email_expire_day.into());
    let user_id = UserQueries::get_user_id(&body.email, &state.storage.postgres.pool).await?;
    println!("Aim: {:?}", CONFIG.app_url);

    let url = format!("{}?token={}", CONFIG.app_url, token);
    print!("Hello : {:?}", url);

    state
        .storage
        .postgres
        .with_transaction(|tx| {
            Box::pin(async move {
                TokenQueries::create_verification_token(
                    user_id,
                    &token,
                    "email_verification",
                    expiry,
                    tx,
                )
                .await?;
                Ok(())
            })
        })
        .await?;

    // send email
    let email = Email {
        recipient: Mailbox::new(Some("".to_owned()), body.email.parse::<Address>()?),
        sender: Mailbox::new(
            Some("Info".to_owned()),
            CONFIG.smtp_email.parse::<Address>()?,
        ),
        subject: String::from("Verification Email"),
        header: ContentType::TEXT_HTML,
        body: verify_email_body(&url)?,
    };

    state.smtp.send_email(email)?;

    Ok(ApiResponse::new(
        "success",
        &format!("Verification email sent."),
        StatusCode::OK,
        "",
    ))
}

pub async fn verify_email(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<VerifyQuery>,
) -> Result<impl IntoResponse> {
    println!("{:?}", query.token);

    Ok(ApiResponse::new(
        "success",
        &format!("Login."),
        StatusCode::CREATED,
        "",
    ))
}

pub async fn send_reset_password(
    State(_state): State<Arc<AppState>>,
    // Json(user): Json<AuthUser>,
) -> Result<impl IntoResponse> {
    println!("login user");

    Ok(ApiResponse::new(
        "success",
        &format!("Login."),
        StatusCode::CREATED,
        "",
    ))
}

pub async fn reset_password(
    State(_state): State<Arc<AppState>>,
    // Json(user): Json<AuthUser>,
) -> Result<impl IntoResponse> {
    println!("login user");

    Ok(ApiResponse::new(
        "success",
        &format!("Login."),
        StatusCode::CREATED,
        "",
    ))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    // Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    let cookie = jar
        .get("refresh_token")
        .ok_or_else(|| Error::CustomError("Refresh token not found.".into()))?;

    let token = cookie.value().to_owned();

    TokenQueries::delete_refresh_token(&token, &state.storage.postgres.pool).await?;

    let jar = jar.remove(Cookie::build("refresh_token").path("/").build());

    Ok((
        jar,
        ApiResponse::new(
            "success",
            &format!("Logout successuful."),
            StatusCode::OK,
            "",
        ),
    ))
}
