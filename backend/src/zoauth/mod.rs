pub mod google;
pub mod microsoft;
pub mod notion;
pub mod slack;

use crate::{
    oauth::{
        google::client::GoogleClient, microsoft::client::MicrosoftClient,
        notion::client::NotionClient, slack::client::SlackClient,
    },
    Result,
};

#[derive(Debug, Clone)]
pub struct ServicesClient {
    pub microsoft: MicrosoftClient,
    pub google: GoogleClient,
    pub slack: SlackClient,
    pub notion: NotionClient,
}

impl ServicesClient {
    pub async fn new() -> Result<Self> {
        Ok(ServicesClient {
            microsoft: MicrosoftClient::new()?,
            google: GoogleClient::new()?,
            slack: SlackClient::new()?,
            notion: NotionClient::new()?,
        })
    }
}
