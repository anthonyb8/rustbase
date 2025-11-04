use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use tracing::info;

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct UserResponse {
    id: i32,
    email: String,
    authenticator_mfa_enabled: bool,
    is_verified: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserResponse {
    pub async fn read_by_id(pool: &PgPool, id: i32) -> Result<UserResponse> {
        info!("Retrieving instrument: {:?}", id);

        let user: UserResponse = sqlx::query_as(
            r#"
            SELECT id,email,authenticator_mfa_enabled,is_verified,created_at,updated_at FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        info!("Successfully fetched user : {}", id);

        Ok(user)
    }
}
