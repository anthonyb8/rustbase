use crate::config::CONFIG;
use crate::data::Objects;
use crate::error::{Error, Result};
use object_store::path::Path;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::{PgConnectOptions, PgPool};
use sqlx::types::Uuid;
use sqlx::ConnectOptions;
use sqlx::{Postgres, Transaction};
use std::ops::DerefMut;
use std::pin::Pin;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PostgresClient {
    pub pool: PgPool,
}

impl PostgresClient {
    pub async fn new() -> Result<Self> {
        let config = &*CONFIG;

        Ok(PostgresClient {
            pool: init_db(&config.postgres_url).await?,
        })
    }

    pub async fn with_transaction<T, F>(&self, operation: F) -> Result<T>
    where
        T: Send,
        F: for<'a> FnOnce(
            &'a mut sqlx::Transaction<'_, sqlx::Postgres>,
        ) -> Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>,
    {
        let mut tx = self.pool.begin().await?;
        match operation(&mut tx).await {
            Ok(result) => {
                tx.commit().await?;
                Ok(result)
            }
            Err(error) => {
                let _ = tx.rollback().await;
                Err(error)
            }
        }
    }

    pub async fn with_pool<T, F>(&self, operation: F) -> Result<T>
    where
        T: Send,
        F: for<'a> FnOnce(&'a PgPool) -> Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>,
    {
        let res = operation(&self.pool).await;
        match res {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        }
    }

    pub async fn start_transaction(&self) -> Result<Transaction<'_, Postgres>> {
        self.pool
            .begin()
            .await
            .map_err(|_| Error::CustomError("Failed to connect to database.".into()))
    }

    // Object metadate
    pub async fn upload_object(&self, object: &Objects) -> Result<()> {
        let mut tx = self.start_transaction().await?;
        let query = r#"
            INSERT INTO objects (user_id, key, filename)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, filename) DO UPDATE 
            SET key = EXCLUDED.key
        "#;

        match sqlx::query(query)
            .bind(&object.user_id)
            .bind(&object.key)
            .bind(&object.filename)
            .execute(tx.deref_mut())
            .await
        {
            Ok(_) => {
                tx.commit().await?;
                Ok(())
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e.into())
            }
        }
    }

    pub async fn get_object(&self, path: &Path) -> Result<Objects> {
        let object: Objects = sqlx::query_as(
            r#"
            SELECT *
            FROM objects 
            WHERE key=$1
            LIMIT 1
        "#,
        )
        .bind(path.to_string())
        .fetch_one(&self.pool)
        .await?;

        Ok(object)
    }

    pub async fn delete_object(&self, key: &str) -> Result<()> {
        let mut tx = self.start_transaction().await?;
        let query = r#"
            DELETE FROM objects 
            WHERE key=$1 
        "#;

        match sqlx::query(query).bind(key).execute(tx.deref_mut()).await {
            Ok(_) => {
                tx.commit().await?;
                Ok(())
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e.into())
            }
        }
    }

    pub async fn list_user_objects(&self, user_id: Uuid) -> Result<Vec<Objects>> {
        let objects: Vec<Objects> = sqlx::query_as(
            r#"
            SELECT * 
            FROM objects 
            WHERE user_id = $1
        "#,
        )
        .bind(&user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(objects)
    }
}

// Improved init function
pub async fn init_db(database_url: &str) -> Result<PgPool> {
    let mut opts: PgConnectOptions = database_url.parse()?;
    opts = opts
        .log_slow_statements(log::LevelFilter::Debug, Duration::from_secs(1))
        .disable_statement_logging() // Only log slow queries
        .application_name("your_app_name"); // Good for monitoring

    let db_pool = PgPoolOptions::new()
        .max_connections(20) // 100 might be too high, start with 20
        .min_connections(5) // Maintain minimum connections
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600)) // 10 minutes
        .max_lifetime(Duration::from_secs(1800)) // 30 minutes
        .connect_with(opts)
        .await?;

    Ok(db_pool)
}
