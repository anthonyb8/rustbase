use crate::config::CONFIG;
use crate::data::AuthorizationFlow;
use crate::error::Result;
use oauth2::{basic::BasicClient, TokenResponse};
use oauth2::{reqwest, AccessToken, PkceCodeChallenge, PkceCodeVerifier, RefreshToken};
use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl, Scope};
use sqlx::types::Uuid;

#[derive(Debug, Clone)]
pub struct MicrosoftOauth {
    http_client: oauth2::reqwest::Client,
    redirect_uri: RedirectUrl,
}

impl MicrosoftOauth {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let redirect_uri =
            RedirectUrl::new(format!("{}/auth/oauth/microsoft/callback", CONFIG.app_url))?;

        Ok(Self {
            http_client,
            redirect_uri,
        })
    }
    /// Start a new OAuth authorization flow
    pub fn get_authorization_url(&self, id: Uuid) -> AuthorizationFlow {
        let csrf_state = CsrfToken::new_random();
        let composite_state = format!("{}:{}", id, csrf_state.secret());
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let client = BasicClient::new(CONFIG.microsoft_client_id.clone()) // ← Fixed: was slack_client_id
            .set_client_secret(CONFIG.microsoft_client_secret.clone())
            .set_auth_uri(CONFIG.microsoft_auth_url.clone())
            .set_token_uri(CONFIG.microsoft_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone());

        // Generate the authorization URL
        let (authorize_url, _csrf_state) = client
            .authorize_url(|| CsrfToken::new(composite_state))
            // Microsoft Graph scopes
            .add_scope(Scope::new(
                "https://graph.microsoft.com/Mail.Read".to_string(),
            ))
            .add_scope(Scope::new(
                "https://graph.microsoft.com/Mail.ReadWrite".to_string(),
            ))
            .add_scope(Scope::new(
                "https://graph.microsoft.com/Mail.Send".to_string(),
            ))
            // .add_scope(Scope::new("offline_access".to_string())) // For refresh token
            .set_pkce_challenge(pkce_challenge)
            .url();

        println!("Open this URL in your browser:\n{authorize_url}\n");

        AuthorizationFlow {
            authorize_url,
            csrf_state,
            pkce_verifier: Some(pkce_verifier),
        }
    }

    /// Exchange the authorization code for a token
    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier, // ← Microsoft DOES use PKCE!
    ) -> Result<(AccessToken, Option<RefreshToken>)> {
        let client = BasicClient::new(CONFIG.microsoft_client_id.clone())
            .set_client_secret(CONFIG.microsoft_client_secret.clone())
            .set_auth_uri(CONFIG.microsoft_auth_url.clone())
            .set_token_uri(CONFIG.microsoft_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone());

        let token = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier) // ← Microsoft requires PKCE!
            .request_async(&self.http_client)
            .await?;

        let access_token = token.access_token();
        let refresh_token = token.refresh_token();

        Ok((access_token.to_owned(), refresh_token.map(|t| t.to_owned())))
    }

    pub async fn refresh_token(&self, refresh_token: RefreshToken) -> Result<AccessToken> {
        let client = BasicClient::new(CONFIG.microsoft_client_id.clone()) // ← Fixed: was slack
            .set_client_secret(CONFIG.microsoft_client_secret.clone()) // ← Fixed: was slack
            .set_auth_uri(CONFIG.microsoft_auth_url.clone()) // ← Fixed: was slack
            .set_token_uri(CONFIG.microsoft_token_url.clone()) // ← Fixed: was slack
            .set_redirect_uri(self.redirect_uri.clone());

        let token = client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.http_client)
            .await?;

        Ok(token.access_token().to_owned())
    }
}
