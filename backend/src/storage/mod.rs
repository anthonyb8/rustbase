pub mod object;
pub mod postgres;
pub mod redis;

use axum::body::Bytes;
use object_store::path::Path;

use crate::{
    config::CONFIG,
    data::Objects,
    storage::{object::ObjectClient, postgres::PostgresClient, redis::RedisClient},
    Result,
};

#[derive(Debug)]
pub struct StorageClient {
    pub postgres: PostgresClient,
    pub redis: RedisClient,
    pub object: ObjectClient,
}

impl StorageClient {
    pub async fn new() -> Result<Self> {
        let config = &*CONFIG;

        Ok(StorageClient {
            postgres: PostgresClient::new().await?,
            redis: RedisClient::new().await?,
            object: ObjectClient::new().await?,
        })
    }

    pub async fn upload_file(&self, object: &Objects, path: &Path, file: &Bytes) -> Result<()> {
        match self.object.upsert(&path, &file).await {
            Ok(()) => match self.postgres.upload_object(object).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_file(&self, path: &Path) -> Result<(Objects, Bytes)> {
        let metadata = match self.postgres.get_object(path).await {
            Ok(obj) => obj,
            Err(e) => return Err(format!("Error getting metadata: {}", e).into()),
        };
        let file = self.object.get(path).await?;

        return Ok((metadata, file));
    }

    pub async fn delete_file(&self, path: &Path) -> Result<()> {
        match self.postgres.delete_object(&path.to_string()).await {
            Ok(()) => match self.object.delete(path).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list_files(&self, user_id: i32) -> Result<Vec<Objects>> {
        self.postgres.list_user_objects(user_id).await
    }
}
