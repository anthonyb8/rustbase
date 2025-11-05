use crate::{Error, Result};
use axum::{body::Bytes, extract::Multipart};
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeVerifier};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Postgres, Transaction};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flow {
    pub csrf_state: CsrfToken,
    pub pkce_verifier: Option<PkceCodeVerifier>,
}

#[derive(Debug, Serialize)]
pub struct AuthorizationFlow {
    pub authorize_url: Url,
    pub csrf_state: CsrfToken,
    pub pkce_verifier: Option<PkceCodeVerifier>,
}

#[derive(Debug, Deserialize)]
pub struct AuthCode {
    pub code: AuthorizationCode,
    pub pkce_verifier: Option<PkceCodeVerifier>,
}

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
