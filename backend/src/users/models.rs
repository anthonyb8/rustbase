use crate::{crypt::hash::hash_password, Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize)]
pub struct UpdateEmailPayload {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordPayload {
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

impl User {
    pub fn new(
        id: Option<i32>,
        email: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
    ) -> Result<User> {
        Ok(User {
            id,
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password_hash: hash_password(password)?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ReturnUser {
    pub id: i32,
    pub email: String,
    pub is_verified: bool,
}

// Handlers
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl TryInto<User> for Credentials {
    type Error = Error;

    fn try_into(self) -> Result<User> {
        Ok(User::new(
            None,
            &self.email,
            &self.first_name,
            &self.last_name,
            &self.password,
        )?)
    }
}

// #[derive(sqlx::FromRow, Clone, Serialize, Deserialize, Debug)]
// pub struct User {
//     id: Option<i32>,
//     email: String,
//     password_hash: String,
//     authenticator_mfa_enabled: bool,
//     mfa_secret: String,
//     is_verified: bool,
//     created_at: DateTime<Utc>,
//     updated_at: DateTime<Utc>,
// }
// impl User {
//     pub async fn create(&self, tx: &mut Transaction<'_, Postgres>) -> Result<i32> {
//         println!("hello");
//         info!("Inserting new instrument: {:?}", self);
//
//         let result = sqlx::query(
//             r#"
//             INSERT INTO users (email, password_hash, is_verified)
//             VALUES ($1, $2, $3)
//             RETURNING id
//             "#,
//         )
//         .bind(&self.email)
//         .bind(&self.password_hash)
//         .bind(&self.is_verified)
//         .fetch_one(tx.deref_mut())
//         .await?;
//
//         let id: i32 = result.try_get("id")?;
//
//         info!("Successfully created user with id: {}", id);
//         Ok(id)
//     }
// }
// #[async_trait]
// pub trait UserQueries: Sized {
//     // async fn register(&self, tx: &mut Transaction<'_, Postgres>) -> Result<i32>;
//     async fn create(&self, tx: &mut Transaction<'_, Postgres>) -> Result<i32>;
//     // async fn read_by_id(pool: &PgPool, id: i32) -> Result<User>;
//     // async fn update(&self, tx: &mut Transaction<'_, Postgres>) -> Result<()>;
//     // async fn delete(tx: &mut Transaction<'_, Postgres>, id: i32) -> Result<()>;
// }
//
// #[async_trait]
// impl UserQueries for User {}
//     async fn read(pool: &PgPool, ticker: &str, dataset: Dataset) -> Result<Vec<Instrument>> {
//         info!("Retrieving instrument: {:?}", ticker);
//
//         let query = format!(
//             r#"
//             SELECT * FROM {}
//             WHERE ticker = $1
//             "#,
//             dataset.as_str()
//         );
//         let rows = sqlx::query(&query).bind(ticker).fetch_all(pool).await?;
//
//         let mut instruments = Vec::new();
//         for row in &rows {
//             instruments.push(Instrument::from_row(row)?);
//         }
//
//         info!("Successfully fetched {} instruments", instruments.len());
//
//         Ok(instruments)
//     }
//
//
//     async fn update(&self, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
//         let id = self
//             .instrument_id
//             .ok_or_else(|| Error::CustomError("Instrument_id cannot be None.".into()))?;
//
//         info!("Updating instrument with id {}.", id);
//
//         let query = format!(
//             r#"
//             UPDATE {}
//             SET ticker=$1, name=$2, vendor=$3, vendor_data=$4, last_available=$5, first_available=$6, expiration_date=$7,is_continuous=$8, active=$9
//             WHERE instrument_id = $10
//             "#,
//             self.dataset.as_str()
//         );
//
//         let _ = sqlx::query(&query)
//             .bind(&self.ticker)
//             .bind(&self.name)
//             .bind(&self.vendor.as_str())
//             .bind(self.vendor_data as i64)
//             .bind(self.last_available as i64)
//             .bind(self.first_available as i64)
//             .bind(self.expiration_date as i64)
//             .bind(self.is_continuous)
//             .bind(self.active)
//             .bind(id as i32)
//             .execute(tx.deref_mut()) // Borrow tx mutably
//             .await?;
//
//         info!("Successfully updated instrument with id {}", id);
//
//         Ok(())
//     }
//
//     async fn delete(tx: &mut Transaction<'_, Postgres>, id: i32) -> Result<()> {
//         info!("Deleting instrument with id {}", id);
//         let _ = sqlx::query(
//             r#"
//             DELETE FROM instrument WHERE id = $1
//             "#,
//         )
//         .bind(&id)
//         .execute(tx.deref_mut())
//         .await?;
//
//         info!("Successfully deleted instrument with id {}", id);
//
//         Ok(())
//     }
// }
//
// pub async fn vendor_list_instruments(
//     pool: &PgPool,
//     vendor: Vendors,
//     dataset: Dataset,
// ) -> Result<Vec<Instrument>> {
//     info!("Fetching list instruments fo vendor : {}", vendor);
//
//     let query = format!(
//         r#"
//         SELECT * FROM {}
//         WHERE vendor = $1
//         "#,
//         dataset.as_str()
//     );
//
//     let rows = sqlx::query(&query)
//         .bind(vendor.as_str())
//         .fetch_all(pool)
//         .await?;
//
//     let mut instruments = Vec::new();
//     for row in &rows {
//         instruments.push(Instrument::from_row(row)?);
//     }
//
//     info!("Successfully fetched {} instruments", vendor);
//
//     Ok(instruments)
// }
//
// pub async fn dataset_list_instruments(pool: &PgPool, dataset: Dataset) -> Result<Vec<Instrument>> {
//     info!("Fetching list instruments for dataset : {}", dataset);
//
//     let query = format!(
//         r#"
//         SELECT * FROM {}
//         "#,
//         dataset.as_str()
//     );
//
//     let rows = sqlx::query(&query).fetch_all(pool).await?;
//
//     let mut instruments = Vec::new();
//     for row in &rows {
//         instruments.push(Instrument::from_row(row)?);
//     }
//
//     info!("Successfully fetched {} instruments", dataset);
//
//     Ok(instruments)
// }
//
// pub async fn query_symbols_map(
//     pool: &PgPool,
//     tickers: &Vec<String>,
//     dataset: Dataset,
// ) -> Result<SymbolMap> {
//     info!("Fetching symbols map.");
//
//     let query = format!(
//         r#"
//         SELECT instrument_id, ticker
//         FROM {}
//         WHERE ticker = ANY($1)
//         "#,
//         dataset.as_str()
//     );
//
//     let rows = sqlx::query(&query)
//         .bind(tickers)
//         .fetch_all(pool) // Use fetch_optional instead of fetch_one
//         .await?;
//
//     let mut map = SymbolMap::new();
//
//     for row in rows {
//         let id: i32 = row.try_get("instrument_id")?;
//         let ticker: String = row.try_get("ticker")?;
//         map.add_instrument(&ticker, id as u32);
//     }
//
//     Ok(map)
// }

// #[cfg(test)]
// mod test {
//     use std::str::FromStr;
//
//     use super::*;
//     use anyhow::Result;
//     use dbn;
//     use dotenv;
//     use mbinary::symbols::SymbolMap;
//     use mbinary::vendors::{DatabentoData, VendorData};
//     use serial_test::serial;
//     use sqlx::postgres::PgPoolOptions;
//     use sqlx::postgres::{PgConnectOptions, PgPool};
//     use sqlx::ConnectOptions;
//     use std::time::Duration;
//
//     // Init Databases
//     pub async fn init_db() -> Result<PgPool> {
//         let database_url = std::env::var("HISTORICAL_DATABASE_URL")?;
//
//         // URL connection string
//         let mut opts: PgConnectOptions = database_url.parse()?;
//         opts = opts.log_slow_statements(log::LevelFilter::Debug, Duration::from_secs(1));
//
//         let db_pool = PgPoolOptions::new()
//             .max_connections(100)
//             .connect_with(opts)
//             .await?;
//         Ok(db_pool)
//     }
//     pub async fn create_instrument_dummy(ticker: &str) -> anyhow::Result<i32> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut transaction = pool.begin().await.expect("Error settign up database.");
//
//         let schema = dbn::Schema::from_str("mbp-1")?;
//         let dataset = dbn::Dataset::from_str("GLBX.MDP3")?;
//         let stype = dbn::SType::from_str("raw_symbol")?;
//         let vendor_data = VendorData::Databento(DatabentoData {
//             schema,
//             dataset,
//             stype,
//         });
//
//         let instrument = Instrument::new(
//             None,
//             ticker,
//             "name",
//             Dataset::Futures,
//             Vendors::Databento,
//             vendor_data.encode(),
//             1704672000000000000,
//             1704672000000000000,
//             0,
//             false,
//             true,
//         );
//         let id = instrument
//             .create(&mut transaction)
//             .await
//             .expect("Error inserting symbol.");
//
//         let _ = transaction.commit().await;
//
//         Ok(id)
//     }
//
//     #[sqlx::test]
//     #[serial]
//     // #[ignore]
//     async fn test_create_instrument() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.expect("Error on creating pool");
//         let mut transaction = pool.begin().await.expect("Error settign up database.");
//
//         let schema = dbn::Schema::from_str("mbp-1")?;
//         let dataset = dbn::Dataset::from_str("GLBX.MDP3")?;
//         let stype = dbn::SType::from_str("raw_symbol")?;
//         let vendor_data = VendorData::Databento(DatabentoData {
//             schema,
//             dataset,
//             stype,
//         });
//
//         let ticker = "AAPL";
//         let name = "Apple Inc.";
//         let instrument = Instrument::new(
//             None,
//             ticker,
//             name,
//             Dataset::Futures,
//             Vendors::Databento,
//             vendor_data.encode(),
//             1704672000000000000,
//             1704672000000000000,
//             0,
//             false,
//             true,
//         );
//
//         // Test
//         let result = instrument
//             .create(&mut transaction)
//             .await
//             .expect("Error inserting symbol.");
//
//         // Validate
//         assert!(result > 0);
//
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     async fn test_read_instrument_by_id() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut transaction = pool.begin().await.expect("Error settign up database.");
//
//         let schema = dbn::Schema::from_str("mbp-1")?;
//         let dataset = dbn::Dataset::from_str("GLBX.MDP3")?;
//         let stype = dbn::SType::from_str("raw_symbol")?;
//         let vendor_data = VendorData::Databento(DatabentoData {
//             schema,
//             dataset,
//             stype,
//         });
//
//         let ticker = "AAPL";
//         let name = "Apple Inc.";
//         let instrument = Instrument::new(
//             None,
//             ticker,
//             name,
//             Dataset::Futures,
//             Vendors::Databento,
//             vendor_data.encode(),
//             1704672000000000000,
//             1704672000000000000,
//             0,
//             false,
//             true,
//         );
//         let id = instrument
//             .create(&mut transaction)
//             .await
//             .expect("Error inserting symbol.");
//         let _ = transaction.commit().await;
//
//         // Test
//         let vec = Instrument::read_by_id(&pool, id)
//             .await
//             .expect("Error getting symbols map.");
//
//         // Validate
//         assert!(vec.len() > 0);
//
//         // Cleanup
//         let mut transaction = pool
//             .begin()
//             .await
//             .expect("Error setting up test transaction.");
//         Instrument::delete(&mut transaction, id)
//             .await
//             .expect("Error on delete.");
//         let _ = transaction.commit().await;
//
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     async fn test_read_instrument() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut transaction = pool.begin().await.expect("Error settign up database.");
//
//         let schema = dbn::Schema::from_str("mbp-1")?;
//         let dataset = dbn::Dataset::from_str("GLBX.MDP3")?;
//         let stype = dbn::SType::from_str("raw_symbol")?;
//         let vendor_data = VendorData::Databento(DatabentoData {
//             schema,
//             dataset,
//             stype,
//         });
//
//         let ticker = "AAPL";
//         let name = "Apple Inc.";
//         let instrument = Instrument::new(
//             None,
//             ticker,
//             name,
//             Dataset::Futures,
//             Vendors::Databento,
//             vendor_data.encode(),
//             1704672000000000000,
//             1704672000000000000,
//             0,
//             false,
//             true,
//         );
//
//         let id = instrument
//             .create(&mut transaction)
//             .await
//             .expect("Error inserting symbol.");
//         let _ = transaction.commit().await;
//
//         // Test
//         let vec = Instrument::read(&pool, "AAPL", Dataset::Futures)
//             .await
//             .expect("Error getting symbols map.");
//
//         // Validate
//         assert!(vec.len() > 0);
//
//         // Cleanup
//         let mut transaction = pool
//             .begin()
//             .await
//             .expect("Error setting up test transaction.");
//         Instrument::delete(&mut transaction, id)
//             .await
//             .expect("Error on delete.");
//         let _ = transaction.commit().await;
//
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     // #[ignore]
//     async fn update_instrument() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut transaction = pool.begin().await.expect("Error settign up database.");
//
//         let schema = dbn::Schema::from_str("mbp-1")?;
//         let dataset = dbn::Dataset::from_str("GLBX.MDP3")?;
//         let stype = dbn::SType::from_str("raw_symbol")?;
//         let vendor_data = VendorData::Databento(DatabentoData {
//             schema,
//             dataset,
//             stype,
//         });
//
//         let ticker = "AAPL";
//         let name = "Apple Inc.";
//         let instrument = Instrument::new(
//             None,
//             ticker,
//             name,
//             Dataset::Equities,
//             Vendors::Databento,
//             vendor_data.encode(),
//             1704672000000000000,
//             1704672000000000000,
//             0,
//             false,
//             true,
//         );
//         let id = instrument
//             .create(&mut transaction)
//             .await
//             .expect("Error inserting symbol.");
//         let _ = transaction.commit().await;
//
//         // Test
//         let new_ticker = "TSLA";
//         let new_name = "Tesla Inc.";
//         let new_instrument = Instrument::new(
//             Some(id as u32),
//             new_ticker,
//             new_name,
//             Dataset::Equities,
//             Vendors::Databento,
//             vendor_data.encode(),
//             1704672000000000001,
//             1704672000000000000,
//             0,
//             false,
//             true,
//         );
//         let mut transaction = pool.begin().await.expect("Error settign up database.");
//         let result = new_instrument
//             .update(&mut transaction)
//             .await
//             .expect("Error updating instrument.");
//         let _ = transaction.commit().await;
//
//         // Validate
//         assert_eq!(result, ());
//
//         // Cleanup
//         let mut transaction = pool
//             .begin()
//             .await
//             .expect("Error setting up test transaction.");
//         Instrument::delete(&mut transaction, id)
//             .await
//             .expect("Error on delete.");
//         let _ = transaction.commit().await;
//
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     async fn test_list_vendors() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut ids = Vec::new();
//
//         let ticker1 = "HEJ4-2";
//         ids.push(create_instrument_dummy(ticker1).await?);
//
//         let ticker2 = "HEF4-2";
//         ids.push(create_instrument_dummy(ticker2).await?);
//
//         // Test
//         let vec = vendor_list_instruments(&pool, Vendors::Databento, Dataset::Futures)
//             .await
//             .expect("Error getting symbols map.");
//
//         // Validate
//         assert!(vec.len() > 0);
//
//         // Cleanup
//         for id in ids {
//             let mut transaction = pool
//                 .begin()
//                 .await
//                 .expect("Error setting up test transaction.");
//             Instrument::delete(&mut transaction, id)
//                 .await
//                 .expect("Error on delete.");
//             let _ = transaction.commit().await;
//         }
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     async fn test_list_dataset() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut ids = Vec::new();
//
//         let ticker1 = "HEJ4-2";
//         ids.push(create_instrument_dummy(ticker1).await?);
//
//         let ticker2 = "HEF4-2";
//         ids.push(create_instrument_dummy(ticker2).await?);
//
//         // Test
//         let vec = dataset_list_instruments(&pool, Dataset::Futures)
//             .await
//             .expect("Error getting symbols map.");
//
//         // Validate
//         assert!(vec.len() > 0);
//
//         // Cleanup
//         for id in ids {
//             let mut transaction = pool
//                 .begin()
//                 .await
//                 .expect("Error setting up test transaction.");
//             Instrument::delete(&mut transaction, id)
//                 .await
//                 .expect("Error on delete.");
//             let _ = transaction.commit().await;
//         }
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     async fn test_query_symbol_map() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut ids = Vec::new();
//         let tickers = vec![
//             "ZC.n.0".to_string(),
//             "GF.n.0".to_string(),
//             "LE.n.0".to_string(),
//             "ZS.n.0".to_string(),
//             "ZL.n.0".to_string(),
//             "ZM.n.0".to_string(),
//             "HE.n.0".to_string(),
//             "CL.n.0".to_string(),
//             "CU.n.0".to_string(),
//         ];
//
//         for ticker in &tickers {
//             let id = create_instrument_dummy(ticker).await?;
//             ids.push(id);
//         }
//
//         // Test
//         let map: SymbolMap = query_symbols_map(&pool, &tickers, Dataset::Futures).await?;
//         // println!("{:?}", map);
//
//         // Validate
//         assert_eq!(9, map.map.len());
//
//         // Cleanup
//         for id in ids {
//             let mut transaction = pool
//                 .begin()
//                 .await
//                 .expect("Error setting up test transaction.");
//             Instrument::delete(&mut transaction, id)
//                 .await
//                 .expect("Error on delete.");
//             let _ = transaction.commit().await;
//         }
//
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     async fn test_get_symbol_map_partial() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let mut ids = Vec::new();
//         let tickers = vec!["AAPL".to_string(), "AAPL1".to_string(), "AAPL2".to_string()];
//
//         let id = create_instrument_dummy("AAPL").await?;
//         ids.push(id);
//
//         let id = create_instrument_dummy("AAPL1").await?;
//         ids.push(id);
//
//         // Test
//         let map = query_symbols_map(&pool, &tickers, Dataset::Futures).await?;
//
//         // Validate
//         assert_eq!(2, map.map.len());
//
//         // Cleanup
//         for id in ids {
//             let mut transaction = pool
//                 .begin()
//                 .await
//                 .expect("Error setting up test transaction.");
//             Instrument::delete(&mut transaction, id)
//                 .await
//                 .expect("Error on delete.");
//             let _ = transaction.commit().await;
//         }
//
//         Ok(())
//     }
//
//     #[sqlx::test]
//     #[serial]
//     async fn test_query_symbol_map_none() -> anyhow::Result<()> {
//         dotenv::dotenv().ok();
//         let pool = init_db().await.unwrap();
//         let tickers = vec!["AAPL".to_string(), "AAPL1".to_string(), "AAPL2".to_string()];
//
//         // Test
//         let map = query_symbols_map(&pool, &tickers, Dataset::Futures).await?;
//
//         // Validate
//         assert_eq!(0, map.map.len());
//
//         Ok(())
//     }
// }
