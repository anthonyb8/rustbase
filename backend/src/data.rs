use oauth2::{AuthorizationCode, CsrfToken, PkceCodeVerifier};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flow {
    pub csrf_state: CsrfToken,
    pub pkce_verifier: Option<PkceCodeVerifier>,
}

#[derive(Debug, Serialize)]
pub struct AuthorizationFlow {
    pub authorize_url: Url,
    pub csrf_state: CsrfToken,
    pub pkce_verifier: Option<PkceCodeVerifier>,
}

#[derive(Debug, Deserialize)]
pub struct AuthCode {
    pub code: AuthorizationCode,
    pub pkce_verifier: Option<PkceCodeVerifier>,
}
