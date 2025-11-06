use super::queries::UserQueries;
use crate::crypt::hash::hash_password;
use crate::crypt::jwt::Claims;
use crate::error::Result;
use crate::response::ApiResponse;
use crate::state::AppState;
use crate::users::models::UpdateEmailPayload;
use crate::users::models::UpdatePasswordPayload;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use sqlx::types::Uuid;
use std::sync::Arc;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)?;
    let pool = &state.storage.postgres.pool;

    match UserQueries::get_user(user_id, pool).await {
        Ok(user) => Ok(ApiResponse::new(StatusCode::OK, "success", user)),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_user_email(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateEmailPayload>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)?;
    let mut tx = state.storage.postgres.start_transaction().await?;

    match UserQueries::update_email(&mut tx, user_id, &payload.email).await {
        Ok(()) => {
            tx.commit().await?;

            Ok(ApiResponse::new(
                StatusCode::OK,
                "Email updated successfully",
                "".to_string(),
            ))
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e.into())
        }
    }
}

pub async fn update_user_password(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdatePasswordPayload>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)?;

    let mut tx = state.storage.postgres.start_transaction().await?;
    let hash = &hash_password(&payload.password)?;

    match UserQueries::update_password(&mut tx, user_id, hash).await {
        Ok(()) => {
            tx.commit().await?;

            Ok(ApiResponse::new(
                StatusCode::OK,
                "Password updated successfully",
                "".to_string(),
            ))
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e.into())
        }
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(&claims.sub)?;

    let mut tx = state.storage.postgres.start_transaction().await?;

    match UserQueries::delete(&mut tx, user_id).await {
        Ok(()) => {
            tx.commit().await?;

            Ok(ApiResponse::new(
                StatusCode::NO_CONTENT,
                &format!("Deleted user id {} successfully.", user_id),
                "".to_string(),
            ))
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e.into())
        }
    }
}
