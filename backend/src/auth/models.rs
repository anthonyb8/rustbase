use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct RegisterUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct Code {
    pub code: String,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct NewPassword {
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerifyQuery {
    pub token: String,
}

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct RefreshToken {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
