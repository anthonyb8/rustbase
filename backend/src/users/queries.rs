use crate::{users::models::User, Result};
use sqlx::types::Uuid;
use sqlx::{PgPool, Postgres, Transaction};

pub struct UserQueries;

impl UserQueries {
    pub async fn get_user(id: Uuid, pool: &PgPool) -> Result<User> {
        let user: User = sqlx::query_as(
            r#"
            SELECT id, email, first_name, last_name 
            FROM users 
            WHERE id=$1
            LIMIT 1
        "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
    pub async fn get_user_id(email: &str, pool: &PgPool) -> Result<Uuid> {
        let id: Uuid = sqlx::query_scalar(
            r#"
            SELECT id 
            FROM users 
            WHERE email=$1
            LIMIT 1
        "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;
        Ok(id)
    }

    pub async fn update_password(
        tx: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        password_hash: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE auth_providers  
            SET password_hash=$1 
            WHERE user_id=$2 
            "#,
        )
        .bind(&password_hash)
        .bind(&user_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn update_email(
        tx: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        email: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users  
            SET email=$1 
            WHERE id=$2 
            "#,
        )
        .bind(&email)
        .bind(&user_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn delete(tx: &mut Transaction<'_, Postgres>, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn get_password(email: &str, pool: &PgPool) -> Result<String> {
        let hash: String = sqlx::query_scalar(
            r#"
        SELECT password_hash 
        FROM users 
        WHERE email = $1
        "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(hash)
    }
}
