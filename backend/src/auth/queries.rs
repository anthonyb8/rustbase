use super::models::RefreshToken;
use crate::auth::models::RegisterUser;
use crate::crypt::hash::{hash_password, hash_token};
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{PgPool, Postgres, Transaction};
use std::ops::DerefMut;

pub struct AuthQueries;

impl AuthQueries {
    pub async fn register(user: &RegisterUser, tx: &mut Transaction<'_, Postgres>) -> Result<Uuid> {
        let user_id: Uuid = sqlx::query_scalar(
            r#"
        INSERT INTO users (email, first_name, last_name, is_verified)
        VALUES ($1, $2, $3, false)
        RETURNING id
        "#,
        )
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .fetch_one(tx.deref_mut())
        .await?;

        sqlx::query(
            r#"
            INSERT INTO auth_providers (user_id, provider, password_hash)
            VALUES ($1,'local',$2)
        "#,
        )
        .bind(user_id)
        .bind(hash_password(&user.password)?)
        .execute(tx.deref_mut())
        .await?;

        Ok(user_id)
    }

    pub async fn get_password(email: &str, pool: &PgPool) -> Result<(Uuid, String)> {
        let result: (Uuid, String) = sqlx::query_as::<_, (Uuid, String)>(
            r#"
            SELECT u.id, a.password_hash 
            FROM auth_providers AS a
            JOIN users AS u 
                ON u.id = a.user_id 
            WHERE u.email=$1
            LIMIT 1
        "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn verify_user(user_id: Uuid, pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users 
            SET is_verified=true,
                updated_at=NOW()
            WHERE id=$1
        "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_password(
        user_id: Uuid,
        password: &str,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE auth_providers 
            SET password_hash=$1,
                updated_at=NOW()
            WHERE user_id=$2
            "#,
        )
        .bind(hash_password(password)?)
        .bind(user_id)
        .execute(tx.deref_mut())
        .await?;

        Ok(())
    }

    pub async fn get_email(user_id: Uuid, pool: &PgPool) -> Result<String> {
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
}

pub struct TokenQueries;

impl TokenQueries {
    pub async fn create_refresh_token(
        user_id: Uuid,
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

    pub async fn validate_refresh_token(token: &str, pool: &PgPool) -> Result<Uuid> {
        let result: Uuid = sqlx::query_scalar(
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
}
