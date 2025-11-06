use crate::auth::queries::AuthQueries;
use crate::config::CONFIG;
use crate::crypt::jwt::Claims;
use crate::crypt::tokens::{generate_code, generate_token};
use crate::error::Result;
use crate::response::ApiResponse;
use crate::smtp::messages::{mfa_code_body, reset_password_body, verify_email_body, Email};
use crate::state::AppState;
use crate::users::queries::UserQueries;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::Address;
use serde::Deserialize;
use sqlx::types::Uuid;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct EmailRequest {
    pub email: String,
}

pub async fn verification_email(
    State(state): State<Arc<AppState>>,
    Json(body): Json<EmailRequest>,
) -> Result<impl IntoResponse> {
    let token = generate_token();
    let user_id = UserQueries::get_user_id(&body.email, &state.storage.postgres.pool).await?;

    state
        .storage
        .redis
        .store_token(&token, "verify-email", user_id, 60 * 60 * 24)
        .await?;

    let url = format!("{}/auth/verify-email?token={}", CONFIG.app_url, token);

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
        StatusCode::OK,
        &format!("Verification email sent."),
        "",
    ))
}

pub async fn reset_password_email(
    State(state): State<Arc<AppState>>,
    Json(body): Json<EmailRequest>,
) -> Result<impl IntoResponse> {
    let token = generate_token();
    let user_id = UserQueries::get_user_id(&body.email, &state.storage.postgres.pool).await?;

    state
        .storage
        .redis
        .store_token(&token, "reset-password", user_id, 60 * 60 * 1)
        .await?;

    let url = format!("{}/auth/reset-password?token={}", CONFIG.app_url, token);

    // send email
    let email = Email {
        recipient: Mailbox::new(Some("".to_owned()), body.email.parse::<Address>()?),
        sender: Mailbox::new(
            Some("Info".to_owned()),
            CONFIG.smtp_email.parse::<Address>()?,
        ),
        subject: String::from("Reset Password"),
        header: ContentType::TEXT_HTML,
        body: reset_password_body(&url)?,
    };

    state.smtp.send_email(email)?;

    Ok(ApiResponse::new(
        StatusCode::OK,
        &format!("Reset password sent.."),
        "",
    ))
}

pub async fn mfa_email(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    // generate mfa code
    let code = generate_code(6);
    let user_id = Uuid::parse_str(&claims.sub)?;
    let email = AuthQueries::get_email(user_id, &state.storage.postgres.pool).await?;

    state
        .storage
        .redis
        .store_mfa_code(&code, user_id, 60 * 5)
        .await?;

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

    state.smtp.send_email(email)?;

    Ok(ApiResponse::new(
        StatusCode::OK,
        &format!("MFA Code sent."),
        "",
    ))
}
