use crate::config::CONFIG;
use crate::data::AuthorizationFlow;
use crate::error::Result;
use oauth2::{basic::BasicClient, StandardRevocableToken, TokenResponse};
use oauth2::{reqwest, AccessToken, PkceCodeVerifier, RefreshToken};
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope};

#[derive(Debug, Clone)]
pub struct GoogleOauth {
    http_client: oauth2::reqwest::Client,
    redirect_uri: RedirectUrl,
    revocation_url: RevocationUrl,
}

impl GoogleOauth {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let redirect_uri = RedirectUrl::new(format!("{}/google/oauth/callback", CONFIG.app_url))?;
        let revocation_url =
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())?;

        Ok(Self {
            http_client,
            redirect_uri,
            revocation_url,
        })
    }

    pub fn get_authorization_url(&self, id: i32) -> AuthorizationFlow {
        let csrf_state = CsrfToken::new_random();
        let composite_state = format!("{}:{}", id, csrf_state.secret());
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let client = BasicClient::new(CONFIG.google_client_id.clone())
            .set_client_secret(CONFIG.google_client_secret.clone())
            .set_auth_uri(CONFIG.google_auth_url.clone())
            .set_token_uri(CONFIG.google_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone())
            .set_revocation_url(self.revocation_url.clone());

        let (authorize_url, _) = client
            .authorize_url(|| CsrfToken::new(composite_state))
            .set_pkce_challenge(pkce_challenge)
            .add_extra_param("access_type", "offline")
            .add_extra_param("prompt", "consent")
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/calendar".into(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/gmail.readonly".into(),
            ))
            .add_scope(Scope::new("https://mail.google.com/".into()))
            .url();

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
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<(AccessToken, RefreshToken)> {
        let client = BasicClient::new(CONFIG.google_client_id.clone())
            .set_client_secret(CONFIG.google_client_secret.clone())
            .set_auth_uri(CONFIG.google_auth_url.clone())
            .set_token_uri(CONFIG.google_token_url.clone())
            .set_redirect_uri(self.redirect_uri.clone())
            .set_revocation_url(self.revocation_url.clone());

        let token = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.http_client)
            .await?;

        let access_token = token.access_token();
        let refresh_token = token.refresh_token().unwrap();

        Ok((access_token.to_owned(), refresh_token.to_owned()))
    }

    pub async fn refresh_token(&self) -> Result<()> {
        Ok(())
    }

    /// Revoke a token
    pub async fn revoke_token(&self, token: StandardRevocableToken) -> anyhow::Result<()> {
        let client = BasicClient::new(CONFIG.google_client_id.clone())
            .set_client_secret(CONFIG.google_client_secret.clone())
            .set_auth_uri(CONFIG.google_auth_url.clone())
            .set_token_uri(CONFIG.google_token_url.clone())
            .set_redirect_uri(
                RedirectUrl::new("http://127.0.0.1:8080/callback".to_string()).unwrap(),
            )
            .set_revocation_url(
                RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string()).unwrap(),
            );
        client
            .revoke_token(token)
            .unwrap()
            .request_async(&self.http_client)
            .await?;
        Ok(())
    }
}
