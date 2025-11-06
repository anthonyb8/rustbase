use crate::{Error, Result};
use axum::{body::Bytes, extract::Multipart};
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeVerifier};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};
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
#[derive(Debug, FromRow)]
pub struct Objects {
    pub id: i32,
    pub user_id: Uuid,
    pub key: String,
    pub filename: String,
    // content_type: String,
    // size_bytes: i64,
    // visibility: String,
    // created_at: chrono::DateTime<Utc>,
}
impl Serialize for Objects {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("id", 3)?;
        s.serialize_field("user_id", &self.user_id.to_string())?;
        s.serialize_field("key", &self.key)?;
        s.serialize_field("filename", &self.filename)?;
        s.end()
    }
}

impl Objects {
    pub async fn process_upload(
        mut multipart: Multipart,
        user_id: Uuid,
    ) -> Result<(Objects, Bytes)> {
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
            user_id: user_id,
            key: format!("{}/{}", user_id, filename), // or however you want to generate key
            filename,
            // content_type,
            // size_bytes,
            // visibility: "private".to_string(),
            // created_at: Utc::now(),
        };

        Ok((object, file_bytes))
    }
}
