use crate::error::Result;
use crate::examples::queries::ObjectMetadataQueries;
use crate::examples::queries::Objects;
use crate::response::ApiResponse;
use crate::state::AppState;
use crate::users::queries::UserQueries;
use axum::extract::Multipart;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    multipart: Multipart,
) -> Result<impl IntoResponse> {
    let (object, file) = Objects::handle_upload(multipart, id).await?;
    let path = object_store::path::Path::from(object.key.as_ref());

    match state.storage.object.upsert(&path, &file).await {
        Ok(()) => {
            let mut tx = state.storage.postgres.start_transaction().await?;
            match ObjectMetadataQueries::upsert(&mut tx, &object).await {
                Ok(id) => {
                    tx.commit().await?;
                    Ok(ApiResponse::new(
                        "",
                        "Successfully created file",
                        StatusCode::CREATED,
                        "".to_string(),
                    ))
                }
                Err(e) => {
                    println!("{:?}", e);
                    tx.rollback().await?;
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err(e.into())
        }
    }
}

pub async fn list_files(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    let pool = &state.storage.postgres.pool;

    match ObjectMetadataQueries::list_user_objects(pool, id).await {
        Ok(objs) => Ok(ApiResponse::new(
            "",
            &format!("Successfully created user with id {}", id),
            StatusCode::CREATED,
            objs,
        )),
        Err(e) => {
            println!("{:?}", e);
            Err(e.into())
        }
    }
}
