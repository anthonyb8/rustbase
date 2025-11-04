use crate::config::CONFIG;
use crate::error::Result;
use crate::oauth::ServicesClient;
use crate::smtp::service::EmailService;
use crate::storage::StorageClient;

#[derive(Debug)]
pub struct AppState {
    pub storage: StorageClient,
    pub smtp: EmailService,
    pub services: ServicesClient,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        println!("balls");
        let config = &*CONFIG;

        Ok(AppState {
            storage: StorageClient::new().await?,
            smtp: EmailService::new(
                &config.smtp_email,
                &config.smtp_email_pw,
                &config.smtp_email_relay,
            )?,
            services: ServicesClient::new().await?,
        })
    }
}
