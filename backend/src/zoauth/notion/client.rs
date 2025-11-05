use crate::config::CONFIG;
use crate::data::AuthorizationFlow;
use crate::error::Result;
use oauth2::{basic::BasicClient, TokenResponse};
use oauth2::{reqwest, AccessToken, RefreshToken};
use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl, Scope};

#[derive(Debug, Clone)]
pub struct NotionClient {
    oauth_client: NotionOauth,
    http_client: reqwest::Client,
}

impl NotionClient {
    pub fn new() -> Result<Self> {
        Ok(NotionClient {
            oauth_client: NotionOauth::new()?,
            http_client: reqwest::Client::new(),
        })
    }

    pub async fn get_authorization_url(&self, user_id: i32) -> AuthorizationFlow {
        self.oauth_client.authorization_url(user_id)
    }

    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
    ) -> Result<(AccessToken, Option<RefreshToken>)> {
        self.oauth_client.exchange_code(code).await
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

#[derive(Debug, Clone)]
struct NotionOauth {
    http_client: oauth2::reqwest::Client,
    redirect_uri: RedirectUrl,
}

impl NotionOauth {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let redirect_uri = RedirectUrl::new(format!("{}/notion/oauth/callback", CONFIG.app_url))?;

        Ok(Self {
            http_client,
            redirect_uri,
        })
    }
    /// Start a new OAuth authorization flow
    pub fn authorization_url(&self, id: i32) -> AuthorizationFlow {
        let csrf_state = CsrfToken::new_random();
        let composite_state = format!("{}:{}", id, csrf_state.secret());

        let client = BasicClient::new(CONFIG.notion_client_id.clone()) // ← Fixed: was slack_client_id
            .set_client_secret(CONFIG.notion_client_secret.clone())
            .set_auth_uri(CONFIG.notion_auth_url.clone())
            .set_token_uri(CONFIG.notion_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone());

        let (authorize_url, _csrf_state) = client
            .authorize_url(|| CsrfToken::new(composite_state))
            // Slack scopes - adjust based on what you need
            .add_scope(Scope::new("channels:read".to_string()))
            .add_scope(Scope::new("chat:write".to_string()))
            .add_scope(Scope::new("users:read".to_string()))
            .url();

        AuthorizationFlow {
            authorize_url,
            csrf_state,
            pkce_verifier: None,
        }
    }

    /// Exchange the authorization code for a token
    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
    ) -> Result<(AccessToken, Option<RefreshToken>)> {
        let client = BasicClient::new(CONFIG.notion_client_id.clone())
            .set_client_secret(CONFIG.notion_client_secret.clone())
            .set_auth_uri(CONFIG.notion_auth_url.clone())
            .set_token_uri(CONFIG.notion_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone());

        let token = client
            .exchange_code(code)
            .request_async(&self.http_client)
            .await?;

        let access_token = token.access_token();
        let refresh_token = token.refresh_token();

        Ok((access_token.to_owned(), refresh_token.map(|t| t.to_owned())))
    }

    pub async fn refresh_token(&self, refresh_token: RefreshToken) -> Result<AccessToken> {
        let client = BasicClient::new(CONFIG.notion_client_id.clone()) // ← Fixed: was slack
            .set_client_secret(CONFIG.notion_client_secret.clone()) // ← Fixed: was slack
            .set_auth_uri(CONFIG.notion_auth_url.clone()) // ← Fixed: was slack
            .set_token_uri(CONFIG.notion_token_url.clone()) // ← Fixed: was slack
            .set_redirect_uri(self.redirect_uri.clone());

        let token = client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.http_client)
            .await?;

        Ok(token.access_token().to_owned())
    }
}
