use std::ops::DerefMut;

use crate::{Error, Result};
use axum::{body::Bytes, extract::Multipart};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Postgres, Transaction};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Objects {
    pub id: i32,
    pub user_id: i32,
    pub key: String,
    pub filename: String,
    // content_type: String,
    // size_bytes: i64,
    // visibility: String,
    // created_at: chrono::DateTime<Utc>,
}

impl Objects {
    pub async fn handle_upload(mut multipart: Multipart, id: i32) -> Result<(Objects, Bytes)> {
        // Just get the first/only field
        let field = multipart
            .next_field()
            .await?
            .ok_or_else(|| Error::from("no file uploaded"))?;

        let filename = field
            .file_name()
            .ok_or_else(|| Error::from("missing filename"))?
            .to_string();

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let file_bytes = field.bytes().await?;
        let size_bytes = file_bytes.len() as i64;

        // let file_bytes = file_bytes.ok_or_else(|| Error::from("missing file field"))?;
        let object = Objects {
            id: 0,
            user_id: id,
            key: format!("{}/{}", id, filename), // or however you want to generate key
            filename,
            // content_type,
            // size_bytes,
            // visibility: "private".to_string(),
            // created_at: Utc::now(),
        };

        Ok((object, file_bytes))
    }
}

pub struct ObjectMetadataQueries;

impl ObjectMetadataQueries {
    pub async fn upsert(tx: &mut Transaction<'_, Postgres>, object: &Objects) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO objects (user_id, key, filename)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, filename) DO UPDATE 
            SET key = EXCLUDED.key
        "#,
        )
        .bind(&object.user_id)
        .bind(&object.key)
        .bind(&object.filename)
        // .bind(&object.content_type)
        // .bind(&object.size_bytes)
        // .bind(&object.visibility)
        .execute(tx.deref_mut())
        .await?;

        Ok(())
    }

    pub async fn list_user_objects(pool: &PgPool, user_id: i32) -> Result<Vec<Objects>> {
        let objects: Vec<Objects> = sqlx::query_as(
            r#"
            SELECT * 
            FROM objects 
            WHERE user_id = $1
        "#,
        )
        .bind(&user_id)
        .fetch_all(pool)
        .await?;

        Ok(objects)
    }
}
