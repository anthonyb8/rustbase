use super::models::{AuthUser, EmailMfaCodes, RefreshToken, User};
use crate::crypt::hash::{hash_password, hash_token};
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::ops::DerefMut;
use tracing::info;

pub struct UserQueries;

impl UserQueries {
    pub async fn register(user: &AuthUser, tx: &mut Transaction<'_, Postgres>) -> Result<i32> {
        let result = sqlx::query(
            r#"
        INSERT INTO users (email, password_hash, is_verified)
        VALUES ($1, $2, false)
        RETURNING id
        "#,
        )
        .bind(&user.email)
        .bind(hash_password(&user.password)?)
        .fetch_one(tx.deref_mut())
        .await?;

        let id: i32 = result.try_get("id")?;

        info!("Successfully created user with id: {}", id);
        Ok(id)
    }

    pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<User> {
        let user: User = sqlx::query_as(
            r#"
        SELECT id, email, password_hash, mfa_secret, is_verified 
        FROM users 
        WHERE email = $1
        "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_email(user_id: i32, pool: &PgPool) -> Result<String> {
        let email: String = sqlx::query_scalar(
            r#"
        SELECT email 
        FROM users 
        WHERE id=$1
        "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(email)
    }

    pub async fn get_user_id(email: &str, pool: &PgPool) -> Result<i32> {
        let id: i32 = sqlx::query_scalar(
            r#"
        SELECT id 
        FROM users 
        WHERE email=$1
        "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }
}

pub struct MfaQueries;

impl MfaQueries {
    pub async fn create_mfa_code(
        user_id: i32,
        code: String,
        expires_at: DateTime<Utc>,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO email_mfa_codes (user_id, code_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(hash_token(&code))
        .bind(expires_at)
        .execute(tx.deref_mut())
        .await?;

        Ok(())
    }

    pub async fn get_mfa_code(user_id: i32, code: &str, pool: &PgPool) -> Result<EmailMfaCodes> {
        let mfa_code: EmailMfaCodes = sqlx::query_as(
            r#"
            SELECT id, user_id, code_hash, expires_at, created_at
            FROM email_mfa_codes
            WHERE user_id = $1 
            AND code_hash = $2 
            "#,
        )
        .bind(user_id)
        .bind(hash_token(code))
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::CustomError("Invalid or expired MFA code".into()))?;

        Ok(mfa_code)
    }
}

pub struct TokenQueries;

impl TokenQueries {
    pub async fn create_refresh_token(
        user_id: i32,
        token: &str,
        expiry: DateTime<Utc>,
        pool: &PgPool,
    ) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3 )
        "#,
        )
        .bind(user_id)
        .bind(hash_token(token))
        .bind(expiry)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_refresh_token(
        user_id: i32,
        token: &str,
        pool: &PgPool,
    ) -> Result<RefreshToken> {
        let refresh_token: RefreshToken = sqlx::query_as(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at
            FROM refresh_tokens
            WHERE user_id = $1 
            AND token_hash = $2 
            "#,
        )
        .bind(user_id)
        .bind(hash_token(token))
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::CustomError("Invalid or expired refresh token".into()))?;

        Ok(refresh_token)
    }

    pub async fn delete_refresh_token(token: &str, pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM refresh_tokens 
            WHERE token_hash=$1
        "#,
        )
        .bind(hash_token(token))
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn validate_refresh_token(token: &str, pool: &PgPool) -> Result<i32> {
        let result = sqlx::query_scalar(
            r#"
            SELECT user_id
            FROM refresh_tokens 
            WHERE token_hash=$1
            AND expires_at > NOW()
        "#,
        )
        .bind(hash_token(token))
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn create_verification_token(
        user_id: i32,
        token: &str,
        token_type: &str,
        expiry: DateTime<Utc>,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO verification_tokens (user_id, token_hash, token_type, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
        )
        .bind(user_id)
        .bind(hash_token(token))
        .bind(token_type)
        .bind(expiry)
        .execute(tx.deref_mut())
        .await?;

        Ok(())
    }
}
