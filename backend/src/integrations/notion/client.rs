use crate::error::Result;
use oauth2::reqwest;

#[derive(Debug, Clone)]
pub struct NotionClient {
    http_client: reqwest::Client,
}

impl NotionClient {
    pub fn new() -> Result<Self> {
        Ok(NotionClient {
            http_client: reqwest::Client::new(),
        })
    }

    // data
    pub async fn get_workspace_info(&self, access_token: &str) -> Result<()> {
        let url = "";

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_pages(&self, access_token: &str) -> Result<()> {
        let url = "";

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_page_id(&self, access_token: &str, page_id: i32) -> Result<()> {
        let url = "";

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_root_page(&self, access_token: &str) -> Result<()> {
        let url = "";

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_database(&self, access_token: &str) -> Result<()> {
        let url = "";

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn create_notes_page(&self, access_token: &str) -> Result<()> {
        let url = "";

        let resp = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(())
    }
}
