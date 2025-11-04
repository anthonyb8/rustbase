use crate::crypt::jwt::{decode_jwt, Claims};
use crate::{Error, Result};
use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};
use chrono::Utc;

pub async fn auth_partial_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let token = get_token(&headers)?;
    let claims = validate_jwt(token, false)?;

    request.extensions_mut().insert(claims);
    let response = next.run(request).await;
    Ok(response)
}

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let token = get_token(&headers)?;
    let claims = validate_jwt(token, true)?;

    request.extensions_mut().insert(claims);
    let response = next.run(request).await;
    Ok(response)
}

fn get_token(headers: &HeaderMap) -> Result<&str> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| Error::from("Missing or invalid token"))
}

fn validate_jwt(token: &str, require_mfa: bool) -> Result<Claims> {
    let claims = decode_jwt(token)?;

    if claims.exp <= Utc::now().timestamp() as usize {
        return Err(Error::from("Token expired"));
    }

    if require_mfa && !claims.mfa_verified {
        return Err(Error::from("MFA required"));
    }

    Ok(claims)
}
