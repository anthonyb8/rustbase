use crate::error::{Error, Result};
use dotenv::dotenv;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use once_cell::sync::Lazy;
use std::{env, fmt::Display, str::FromStr};

pub fn get_env<T>(name: &str, default: Option<&str>) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    println!("{}", name);

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
    println!("{:?}", value);

    value
        .parse::<T>()
        .map_err(|e| Error::from(format!("Error parsing '{}': {}", name, e)))
}

// Global config instance - initialized once, read everywhere
pub static CONFIG: Lazy<AppConfig> =
    Lazy::new(|| AppConfig::from_env().expect("Failed to load configuration"));

#[derive(Debug, Clone)]
pub struct AppConfig {
    // JWT
    pub jwt_algorithm: String,
    pub jwt_access_secret: String,
    pub access_token_expire_minutes: u8,
    pub refresh_token_expire_days: u8,
    pub temp_login_expire_minutes: u8,
    pub reset_pw_expire_hours: u8,
    pub verify_email_expire_day: u8,

    // MFA
    pub mfa_secret_key: String,
    pub email_mfa_expire_minutes: u8,

    // CORS
    // pub allowed_origins: Vec<String>,
    // App
    pub app_port: String,
    pub app_url: String,
    pub app_name: String,
    pub debug: bool,
    pub log_file: String,
    pub log_level: String,

    // Email
    pub smtp_email: String,
    pub smtp_email_pw: String,
    pub smtp_email_relay: String,

    // Other
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64, // seconds
    pub idle_timeout: u64,       // seconds
    pub max_lifetime: u64,       // seconds

    //Storage
    pub postgres_url: String,
    pub redis_url: String,
    pub object_url: String,
    pub gcp_path: String,
    pub aws_access: String,
    pub aws_secret: String,

    // Google
    pub google_client_id: ClientId,
    pub google_client_secret: ClientSecret,
    pub google_auth_url: AuthUrl,
    pub google_token_url: TokenUrl,

    // Microsoft
    pub microsoft_client_id: ClientId,
    pub microsoft_client_secret: ClientSecret,
    pub microsoft_auth_url: AuthUrl,
    pub microsoft_token_url: TokenUrl,

    // Slack
    pub slack_client_id: ClientId,
    pub slack_client_secret: ClientSecret,
    pub slack_auth_url: AuthUrl,
    pub slack_token_url: TokenUrl,

    // Notion
    pub notion_client_id: ClientId,
    pub notion_client_secret: ClientSecret,
    pub notion_auth_url: AuthUrl,
    pub notion_token_url: TokenUrl,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let config = AppConfig {
            // JWT
            jwt_algorithm: get_env("JWT_ALGORITHM", None)?,
            jwt_access_secret: get_env("JWT_ACCESS_SECRET", None)?,
            access_token_expire_minutes: get_env("ACCESS_TOKEN_EXPIRE_MINUTES", Some("30"))?,
            refresh_token_expire_days: get_env("REFRESH_TOKEN_EXPIRE_MINUTES", Some("7"))?,
            temp_login_expire_minutes: get_env("TEMP_LONG_EXPIRE_MINUTES", Some("5"))?,
            reset_pw_expire_hours: get_env("RESET_PW_EXPIRE_HOURS", Some("1"))?,
            verify_email_expire_day: get_env("VERIFY_EMAIL_EXPIRE_DAY", Some("1"))?,

            // MFA
            mfa_secret_key: get_env("MFA_ENCRYPTION_KEY", None)?,
            email_mfa_expire_minutes: get_env("EMAIL_MFA_EXPIRE_MINUTES", Some("5"))?,

            // CORS
            // allowed_origins: env::var("ALLOWED_ORIGINS").map_err(|_| Error::from("LOG_FILE"))?,

            // APP
            app_url: env::var("APP_URL")?,
            app_port: get_env("APP_PORT", None)?,
            app_name: get_env("APP_NAME", None)?,
            log_level: get_env("LOG_LEVEL", None)?,
            log_file: get_env("LOG_FILE", None)?,
            debug: get_env("DEBUG", Some("false"))?,

            // Email
            smtp_email: get_env("SMTP_EMAIL", None)?,
            smtp_email_pw: get_env("SMTP_EMAIL_PASSWORD", None)?,
            smtp_email_relay: get_env("SMTP_EMAIL_RELAY", Some("mail.privateemail.com"))?,

            // Other
            max_connections: get_env("MAX_CONNECTIONS", Some("20"))?,
            min_connections: get_env("MIN_CONNECTIONS", Some("5"))?,
            connection_timeout: get_env("CONNECTION_TIMEOUT", Some("30"))?,
            idle_timeout: get_env("IDLE_TIMEOUT", Some("600"))?,
            max_lifetime: get_env("MAX_LIFETIME", Some("1800"))?,

            // Storage
            postgres_url: get_env("POSTGRES_URL", None)?,
            redis_url: get_env("REDIS_URL", None)?,
            object_url: get_env("OBJECT_URL", None)?,
            gcp_path: get_env("GCP_SERVICE_ACCOUNT_PATH", Some(""))?,
            aws_access: get_env("AWS_ACCESS_KEY_ID", Some(""))?,
            aws_secret: get_env("AWS_SECRET_ACCESS_KEY", Some(""))?,

            //OAuth
            google_client_id: ClientId::new(get_env("GOOGLE_CLIENT_ID", None)?),
            google_client_secret: ClientSecret::new(get_env("GOOGLE_CLIENT_SECRET", None)?),
            google_auth_url: AuthUrl::new(
                "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            )?,
            google_token_url: TokenUrl::new(
                "https://www.googleapis.com/oauth2/v3/token".to_string(),
            )?,
            microsoft_client_id: ClientId::new(get_env("MICROSOFT_CLIENT_ID", None)?),
            microsoft_client_secret: ClientSecret::new(get_env("MICROSOFT_CLIENT_SECRET", None)?),
            microsoft_auth_url: AuthUrl::new(
                "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
            )?,
            microsoft_token_url: TokenUrl::new(
                "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
            )?,
            slack_client_id: ClientId::new(get_env("SLACK_CLIENT_ID", None)?),
            slack_client_secret: ClientSecret::new(get_env("SLACK_CLIENT_SECRET", None)?),
            slack_auth_url: AuthUrl::new("https://slack.com/oauth/v2/authorize".to_string())?,
            slack_token_url: TokenUrl::new("https://slack.com/api/oauth.v2.access".to_string())?,
            notion_client_id: ClientId::new(get_env("NOTION_CLIENT_ID", None)?),
            notion_client_secret: ClientSecret::new(get_env("NOTION_CLIENT_SECRET", None)?),
            notion_auth_url: AuthUrl::new("https://api.notion.com/v1/oauth/authorize".to_string())?,
            notion_token_url: TokenUrl::new("https://api.notion.com/v1/oauth/token".to_string())?,
        };
        println!("config loadde");
        Ok(config)
    }
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
