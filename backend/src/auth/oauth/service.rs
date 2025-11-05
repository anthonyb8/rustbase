use crate::{
    auth::oauth::{
        google::client::GoogleOauth, microsoft::client::MicrosoftOauth,
        notion::client::NotionOauth, slack::client::SlackOauth,
    },
    Result,
};

#[derive(Debug, Clone)]
pub struct OAuthClient {
    pub microsoft: MicrosoftOauth,
    pub google: GoogleOauth,
    pub slack: SlackOauth,
    pub notion: NotionOauth,
}

impl OAuthClient {
    pub async fn new() -> Result<Self> {
        Ok(OAuthClient {
            microsoft: MicrosoftOauth::new()?,
            google: GoogleOauth::new()?,
            slack: SlackOauth::new()?,
            notion: NotionOauth::new()?,
        })
    }
}
