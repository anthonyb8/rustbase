use super::queries::UserQueries;
use crate::crypt::hash::hash_password;
use crate::crypt::jwt::Claims;
use crate::error::Result;
use crate::response::ApiResponse;
use crate::state::AppState;
use crate::users::models::Credentials;
use crate::users::models::UpdateEmailPayload;
use crate::users::models::UpdatePasswordPayload;
use crate::users::models::User;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse> {
    let user: User = credentials.try_into()?;
    let mut tx = state.storage.postgres.start_transaction().await?;

    match UserQueries::create(&mut tx, &user).await {
        Ok(id) => {
            tx.commit().await?;
            Ok(ApiResponse::new(
                "",
                &format!("Successfully created user with id {}", id),
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

pub async fn verify_user_email(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let mut tx = state.storage.postgres.start_transaction().await?;

    match UserQueries::verify_user(&mut tx, user_id).await {
        Ok(()) => {
            tx.commit().await?;

            Ok(ApiResponse::new(
                "",
                "Email verified successfully",
                StatusCode::OK,
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

pub async fn update_user_email(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateEmailPayload>,
) -> Result<impl IntoResponse> {
    let mut tx = state.storage.postgres.start_transaction().await?;

    match UserQueries::update_email(&mut tx, user_id, &payload.email).await {
        Ok(()) => {
            tx.commit().await?;

            Ok(ApiResponse::new(
                "",
                "Email updated successfully",
                StatusCode::OK,
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

pub async fn update_user_password(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdatePasswordPayload>,
) -> Result<impl IntoResponse> {
    let mut tx = state.storage.postgres.start_transaction().await?;
    let hash = &hash_password(&payload.password)?;

    match UserQueries::update_password(&mut tx, user_id, hash).await {
        Ok(()) => {
            tx.commit().await?;

            Ok(ApiResponse::new(
                "",
                "Password updated successfully",
                StatusCode::OK,
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

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    let mut tx = state.storage.postgres.start_transaction().await?;

    match UserQueries::delete(&mut tx, id).await {
        Ok(()) => {
            tx.commit().await?;

            Ok(ApiResponse::new(
                "success",
                &format!("Deleted user id {} successfully.", id),
                StatusCode::NO_CONTENT,
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

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    // Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    let pool = &state.storage.postgres.pool;

    match UserQueries::get(pool, id).await {
        Ok(user) => Ok(ApiResponse::new("success", "", StatusCode::OK, user)),
        Err(e) => Err(e.into()),
    }
}
