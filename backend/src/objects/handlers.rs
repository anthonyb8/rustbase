use crate::crypt::jwt::Claims;
use crate::data::Objects;
use crate::error::Result;
use crate::response::ApiResponse;
use crate::state::AppState;
use crate::Error;
use axum::extract::Multipart;
use axum::extract::Query;
use axum::extract::State;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    multipart: Multipart,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)?;

    let (object, file) = Objects::process_upload(multipart, user_id).await?;
    let path = object_store::path::Path::from(object.key.as_ref());

    match state.storage.upload_file(&object, &path, &file).await {
        Ok(_) => Ok(ApiResponse::new(
            StatusCode::CREATED,
            "Successfully created file",
            "".to_string(),
        )),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_file(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let name = params
        .get("name")
        .ok_or_else(|| Error::from("Missing 'name' parameter"))?;
    let key = format!("{}/{}", &claims.sub, name);
    let path = object_store::path::Path::from(key.as_str());

    match state.storage.get_file(&path).await {
        Ok((obj, file)) => {
            let headers = [
                (header::CONTENT_TYPE, "application/octet-stream"),
                (
                    header::CONTENT_DISPOSITION,
                    &format!("attachment; filename=\"{}\"", obj.filename),
                ),
            ];

            Ok((headers, file).into_response())
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_files(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let name = params
        .get("name")
        .ok_or_else(|| Error::from("Missing 'name' parameter"))?;
    let key = format!("{}/{}", &claims.sub, name);
    let path = object_store::path::Path::from(key.as_str());

    match state.storage.delete_file(&path).await {
        Ok(_) => Ok(ApiResponse::new(StatusCode::NO_CONTENT, "", "")),
        Err(e) => Err(e.into()),
    }
}

pub async fn list_files(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)?;

    match state.storage.list_files(user_id).await {
        Ok(objs) => Ok(ApiResponse::new(
            StatusCode::OK,
            &format!("Successfully retrieved list."),
            objs,
        )),
        Err(e) => Err(e.into()),
    }
}
