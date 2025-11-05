use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub mfa_secret: Option<String>,
    pub is_verified: bool,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct Code {
    pub code: String,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct EmailMfaCodes {
    pub id: i32,
    pub user_id: i32,
    pub code_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct RefreshToken {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationTokens {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct VerificationEmail {
    pub email: String,
}

#[derive(Deserialize)]
pub struct VerifyQuery {
    pub token: String,
}
