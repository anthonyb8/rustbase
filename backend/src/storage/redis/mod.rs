use crate::{
    data::{Flow, Token},
    error::Result,
};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use serde_json;

#[derive(Debug)]
pub struct RedisClient {
    pub client: redis::Client,
    conn: MultiplexedConnection,
}

impl RedisClient {
    pub async fn new() -> Result<RedisClient> {
        let client = redis::Client::open("redis://redis/")?;
        let conn = client.get_multiplexed_tokio_connection().await?;
        Ok(RedisClient { client, conn })
    }

    pub async fn connection(&self) -> MultiplexedConnection {
        self.conn.clone()
    }

    pub async fn store_flow(&self, user_id: i32, platform: &str, flow: &Flow) -> Result<()> {
        let key = format!("oauth:{}:{}:flow", user_id, platform);
        let value = serde_json::to_string(flow)?;
        let _: () = self.conn.clone().set(key, value).await?;
        Ok(())
    }

    pub async fn get_flow(&self, user_id: &str, platform: &str) -> Result<Flow> {
        let key = format!("oauth:{}:{}:flow", user_id, platform);
        let value: String = self.conn.clone().get(key).await?;
        let flow: Flow = serde_json::from_str(&value)?;
        Ok(flow)
    }

    pub async fn delete_flow(&self, user_id: &str, platform: &str) -> Result<()> {
        let key = format!("oauth:{}:{}:flow", user_id, platform);
        let _: () = self.conn.clone().del(key).await?;
        Ok(())
    }

    pub async fn store_oauth_tokens(
        &self,
        user_id: &str,
        platform: &str,
        token: &Token,
    ) -> Result<()> {
        let key = format!("oauth:{}:{}", user_id, platform);
        let value = serde_json::to_string(token)?;

        let _: () = self.conn.clone().set(key, value).await?;
        Ok(())
    }

    pub async fn get_oauth_tokens(&self, user_id: i32, platform: &str) -> Result<Token> {
        let key = format!("oauth:{}:{}", user_id, platform);
        let value: String = self.conn.clone().get(key).await?;
        let token: Token = serde_json::from_str(&value)?;

        Ok(token)
    }

    pub async fn delete_oauth_tokens(&self, user_id: &str, platform: &str) -> Result<()> {
        let key = format!("oauth:{}:{}", user_id, platform);
        let _: () = self.conn.clone().del(key).await?;
        Ok(())
    }
}
