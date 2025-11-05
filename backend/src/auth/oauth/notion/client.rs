use crate::config::CONFIG;
use crate::data::AuthorizationFlow;
use crate::error::Result;
use oauth2::{basic::BasicClient, TokenResponse};
use oauth2::{reqwest, AccessToken, RefreshToken};
use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl, Scope};

#[derive(Debug, Clone)]
pub struct NotionOauth {
    http_client: oauth2::reqwest::Client,
    redirect_uri: RedirectUrl,
}

impl NotionOauth {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let redirect_uri =
            RedirectUrl::new(format!("{}/auth/oauth/notion/callback", CONFIG.app_url))?;

        Ok(Self {
            http_client,
            redirect_uri,
        })
    }
    /// Start a new OAuth authorization flow
    pub fn get_authorization_url(&self, id: i32) -> AuthorizationFlow {
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
