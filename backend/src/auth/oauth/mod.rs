use oauth2::AuthorizationCode;
use serde::Deserialize;

pub mod google;
pub mod microsoft;
pub mod notion;
pub mod routes;
pub mod service;
pub mod slack;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    state: String,
    code: AuthorizationCode,
}
