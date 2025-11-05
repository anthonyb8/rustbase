use crate::error::Result;
use oauth2::reqwest;

#[derive(Debug, Clone)]
pub struct SlackClient {
    http_client: reqwest::Client,
}

impl SlackClient {
    pub fn new() -> Result<Self> {
        Ok(SlackClient {
            http_client: reqwest::Client::new(),
        })
    }

    // data
    pub async fn get_user_info(&self, access_token: &str, user_id: i32) -> Result<()> {
        let url = format!("`https://slack.com/api/users.info?user={}", user_id);

        let response = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_workspace_info(&self, access_token: String) -> Result<()> {
        let response = self
            .http_client
            .get("https://slack.com/api/team.info")
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_workspace(&self, access_token: String) -> Result<()> {
        let response = self
            .http_client
            .get("https://slack.com/api/auth.test")
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_channels(&self, access_token: String) -> Result<()> {
        let response = self
            .http_client
            .get("https://slack.com/api/conversations.list?types=public_channel,private_channel&limit=100")
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_messages(&self, access_token: String, channel_id: i32) -> Result<()> {
        let url = format!(
            "https://slack.com/api/conversations.info?channel=${}",
            channel_id
        );
        let response = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn post_message(&self, access_token: String, message: String) -> Result<()> {
        let response = self
            .http_client
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(access_token)
            .body(message)
            .send()
            .await?;
        Ok(())
    }
}
