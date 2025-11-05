use crate::error::Result;
use oauth2::reqwest;

#[derive(Debug, Clone)]
pub struct MicrosoftClient {
    http_client: reqwest::Client,
}

impl MicrosoftClient {
    pub fn new() -> Result<Self> {
        Ok(MicrosoftClient {
            http_client: reqwest::Client::new(),
        })
    }

    // GMAIL
    pub async fn get_outlook_messages(&self, access_token: &str, limit: i32) -> Result<()> {
        // use id to get access token
        let url = format!(
            "https://graph.microsoft.com/v1.0/me/messages?$top={}&$orderby=receivedDateTime desc",
            limit
        );

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;
        // let messages: GmailList = resp.json().await?;

        Ok(())
    }

    pub async fn get_outlook_stats(&self, access_token: &str) -> Result<()> {
        let resp = self
            .http_client
            .get("https://graph.microsoft.com/v1.0/me/messages?$filter=isRead eq false&$count=true")
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_user(&self, access_token: &str) -> Result<()> {
        let resp = self
            .http_client
            .get("https://graph.microsoft.com/v1.0/me")
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }
}
