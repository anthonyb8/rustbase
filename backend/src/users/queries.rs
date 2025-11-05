use crate::{
    users::models::{ReturnUser, User},
    Result,
};
use sqlx::types::Uuid;
use sqlx::{PgPool, Postgres, Transaction};

pub struct UserQueries;

impl UserQueries {
    pub async fn create(tx: &mut Transaction<'_, Postgres>, user: &User) -> Result<Uuid> {
        let id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, first_name, last_name) 
            VALUES ($1, $2, $3) 
            RETURNING id
            "#,
        )
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .fetch_one(&mut **tx)
        .await?;

        Ok(id)
    }

    pub async fn update_password(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
        password_hash: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users  
            SET password_hash=$1 
            WHERE id=$2 
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
        user_id: i32,
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

    pub async fn verify_user(tx: &mut Transaction<'_, Postgres>, user_id: i32) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users  
            SET is_verified=true 
            WHERE id=$1 
            "#,
        )
        .bind(&user_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn delete(tx: &mut Transaction<'_, Postgres>, id: i32) -> Result<()> {
        sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn get(pool: &PgPool, id: i32) -> Result<ReturnUser> {
        let user: ReturnUser = sqlx::query_as(
            r#"
                SELECT id, email, is_verified
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

    pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<ReturnUser> {
        let user: ReturnUser = sqlx::query_as(
            r#"
        SELECT id, email, is_verified 
        FROM users 
        WHERE email = $1
        "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
