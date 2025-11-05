use crate::config::CONFIG;
use crate::data::AuthorizationFlow;
use crate::error::{Error, Result};
use oauth2::{basic::BasicClient, TokenResponse};
use oauth2::{reqwest, AccessToken, RefreshToken};
use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl, Scope};
use reqwest::StatusCode;

#[derive(Debug, Clone)]
pub struct SlackOauth {
    http_client: oauth2::reqwest::Client,
    redirect_uri: RedirectUrl,
}

impl SlackOauth {
    pub fn new() -> Result<Self> {
        let redirect_uri =
            RedirectUrl::new(format!("{}/auth/oauth/slack/callback", CONFIG.app_url))?;
        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Ok(Self {
            http_client,
            redirect_uri,
        })
    }

    /// Start a new OAuth authorization flow
    pub fn get_authorization_url(&self, id: i32) -> AuthorizationFlow {
        let csrf_state = CsrfToken::new_random();
        let composite_state = format!("{}:{}", id, csrf_state.secret());

        let client = BasicClient::new(CONFIG.slack_client_id.clone())
            .set_client_secret(CONFIG.slack_client_secret.clone())
            .set_auth_uri(CONFIG.slack_auth_url.clone())
            .set_token_uri(CONFIG.slack_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone());

        // Generate the authorization URL
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
        let client = BasicClient::new(CONFIG.slack_client_id.clone())
            .set_client_secret(CONFIG.slack_client_secret.clone())
            .set_auth_uri(CONFIG.slack_auth_url.clone())
            .set_token_uri(CONFIG.slack_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone());

        let token = client
            .exchange_code(code)
            .request_async(&self.http_client)
            .await?;

        let access_token = token.access_token();
        let refresh_token = token.refresh_token(); // Slack may or may not return refresh token

        Ok((access_token.to_owned(), refresh_token.map(|t| t.to_owned())))
    }

    pub async fn refresh_token(&self, refresh_token: RefreshToken) -> Result<AccessToken> {
        let client = BasicClient::new(CONFIG.slack_client_id.clone())
            .set_client_secret(CONFIG.slack_client_secret.clone())
            .set_auth_uri(CONFIG.slack_auth_url.clone())
            .set_token_uri(CONFIG.slack_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone());

        let token = client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.http_client)
            .await?;

        Ok(token.access_token().to_owned())
    }

    /// Revoke a token
    pub async fn revoke_token(&self, token: String) -> Result<()> {
        // Slack uses a different revocation endpoint
        let revoke_url = "https://slack.com/api/auth.revoke";

        let params = [("token", token.as_str())];

        let response = self
            .http_client
            .post(revoke_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| Error::CustomError(format!("Revocation request failed: {}", e)))?;

        // let json: serde_json::Value = response
        //     .json()
        //     .await
        //     .map_err(|e| Error::CustomError(format!("Failed to parse response: {}", e)))?;

        if response.status() != StatusCode::OK {
            return Err(Error::CustomError(format!("Revocation failed")));
        }

        Ok(())
    }
}
