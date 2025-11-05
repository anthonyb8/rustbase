use super::data::{EmailMessage, GmailList};
use crate::error::Result;
use oauth2::reqwest;

#[derive(Debug, Clone)]
pub struct GoogleClient {
    http_client: reqwest::Client,
}

impl GoogleClient {
    pub fn new() -> Result<Self> {
        Ok(GoogleClient {
            http_client: reqwest::Client::new(),
        })
    }

    // GMAIL
    pub async fn get_gmail_messages(&self, access_token: &str, limit: i32) -> Result<GmailList> {
        let url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults={}",
            limit
        );

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        let messages: GmailList = resp.json().await?;

        Ok(messages)
    }

    pub async fn get_message_details(
        &self,
        access_token: &str,
        msg_id: &str,
    ) -> Result<EmailMessage> {
        let url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
            msg_id
        );

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        let message: EmailMessage = resp.json().await?;

        Ok(message)
    }

    pub async fn get_user(&self, access_token: &str) -> Result<()> {
        let url = "https://gmail.googleapis.com/gmail/v1/profile";

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }
}
