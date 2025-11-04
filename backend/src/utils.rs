use crate::{Error, Result};
use anyhow::Result as anyResult;
use sqlx::Column;
use sqlx::Row;
use std::{env, fmt::Display, str::FromStr};

/// Used for debugging purposes. ADJUST FOR TESTING
#[allow(dead_code)]
pub fn print_pg_row(row: &sqlx::postgres::PgRow) -> anyResult<()> {
    for column in row.columns() {
        let column_name = column.name();

        // Attempt to get the value as a string for debugging purposes
        let value: anyResult<String, sqlx::Error> = row.try_get(column_name);
        match value {
            Ok(val) => println!("{:?}: {:?}", column_name, val),
            Err(err) => println!("{}: Error fetching value - {:?}", column_name, err),
        }
    }
    Ok(())
}

pub fn get_env<T>(name: &str, default: Option<&str>) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    // Parse value to typ if exists
    let value = match env::var(name) {
        Ok(s) => s,
        Err(_) => match default {
            Some(d) => d.to_string(),
            None => {
                return Err(Error::from(format!(
                    "Environment variable {} not found and no default provided",
                    name
                )))
            }
        },
    };

    value
        .parse::<T>()
        .map_err(|e| Error::from(format!("Error parsing '{}': {}", name, e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env() {
        let x = get_env::<String>("APP_PORT", Some("false")).unwrap();
        println!("{:?}", x);
    }
}
