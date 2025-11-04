use crate::crypt::hash::hash_password;
use crate::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Row, Transaction};
use std::ops::DerefMut;
use tracing::info;

#[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct CreateUser {
    email: String,
    password: String,
}

impl CreateUser {
    pub async fn register(&self, tx: &mut Transaction<'_, Postgres>) -> Result<i32> {
        info!("Inserting new instrument: {:?}", self);

        let result = sqlx::query(
            r#"
            INSERT INTO users (email, password_hash, is_verified)
            VALUES ($1, $2, false)
            RETURNING id
            "#,
        )
        .bind(&self.email)
        .bind(hash_password(&self.password)?)
        .fetch_one(tx.deref_mut())
        .await?;

        let id: i32 = result.try_get("id")?;

        info!("Successfully created user with id: {}", id);
        Ok(id)
    }
}
