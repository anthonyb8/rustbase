use crate::auth::queries::{MfaQueries, TokenQueries, UserQueries};
use crate::config::CONFIG;
use crate::crypt::jwt::Claims;
use crate::crypt::tokens::{generate_code, generate_token};
use crate::error::Result;
use crate::response::ApiResponse;
use crate::smtp::messages::{mfa_code_body, verify_email_body, Email};
use crate::smtp::models::VerificationEmail;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chrono::{Duration, Utc};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::Address;
use std::sync::Arc;

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
