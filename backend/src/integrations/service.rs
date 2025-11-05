use super::{
    google::client::GoogleClient, microsoft::client::MicrosoftClient, notion::client::NotionClient,
    slack::client::SlackClient,
};
use crate::Result;

#[derive(Debug, Clone)]
pub struct IntegrationClient {
    pub microsoft: MicrosoftClient,
    pub google: GoogleClient,
    pub slack: SlackClient,
    pub notion: NotionClient,
}

impl IntegrationClient {
    pub async fn new() -> Result<Self> {
        Ok(IntegrationClient {
            microsoft: MicrosoftClient::new()?,
            google: GoogleClient::new()?,
            slack: SlackClient::new()?,
            notion: NotionClient::new()?,
        })
    }
}
