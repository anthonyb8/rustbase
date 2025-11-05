use crate::data::Objects;
use crate::error::Result;
use crate::response::ApiResponse;
use crate::state::AppState;
use crate::Error;
use axum::extract::Multipart;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::header;
use axum::http::HeaderName;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    multipart: Multipart,
) -> Result<impl IntoResponse> {
    let (object, file) = Objects::handle_upload(multipart, id).await?;
    let path = object_store::path::Path::from(object.key.as_ref());

    match state.storage.upload_file(&object, &path, &file).await {
        Ok(_) => Ok(ApiResponse::new(
            "",
            "Successfully created file",
            StatusCode::CREATED,
            "".to_string(),
        )),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let name = params
        .get("name")
        .ok_or_else(|| Error::from("Missing 'name' parameter"))?;

    let path = object_store::path::Path::from(name.as_str());

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
    Path(id): Path<i32>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let name = params
        .get("name")
        .ok_or_else(|| Error::from("Missing 'name' parameter"))?;

    let path = object_store::path::Path::from(name.as_str());

    match state.storage.delete_file(&path).await {
        Ok(_) => Ok(ApiResponse::new("", "", StatusCode::NO_CONTENT, "")),
        Err(e) => Err(e.into()),
    }
}

pub async fn list_files(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    match state.storage.list_files(id).await {
        Ok(objs) => Ok(ApiResponse::new(
            "",
            &format!("Successfully created user with id {}", id),
            StatusCode::OK,
            objs,
        )),
        Err(e) => Err(e.into()),
    }
}
