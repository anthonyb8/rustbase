pub mod object;
pub mod postgres;
pub mod redis;

use crate::{
    config::CONFIG,
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
}
