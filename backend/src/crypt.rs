use crate::config::CONFIG;
use crate::{Error, Result};

pub mod jwt {
    use super::*;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};
    use std::usize;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Claims {
        pub sub: String,
        pub mfa_verified: bool,
        pub exp: usize,
        pub iat: usize,
    }

    pub fn encode_jwt(user_id: String, mfa_verified: bool) -> Result<String> {
        let now = Utc::now();
        let expiry = if mfa_verified {
            now + Duration::minutes(CONFIG.access_token_expire_minutes.into())
        } else {
            now + Duration::minutes(5)
        };

        let claims = Claims {
            sub: user_id,
            mfa_verified: mfa_verified,
            exp: expiry.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let jwt = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(CONFIG.jwt_access_secret.as_ref()),
        )?;
        Ok(jwt)
    }

    pub fn decode_jwt(token: &str) -> Result<Claims> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(CONFIG.jwt_access_secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(decoded.claims)
    }

    pub fn validate_jwt(token: &str) -> bool {
        match decode_jwt(token) {
            Ok(claims) => claims.exp > Utc::now().timestamp() as usize,
            Err(_) => false,
        }
    }
}

pub mod hash {
    use super::*;
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
    use sha2::{Digest, Sha256};

    // Passwords: Need salt + slow hashing (Argon2)
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(Error::CustomError(e.to_string())),
        }
    }
    // Tokens: Already high entropy, just need fast hashing (SHA-256)
    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn verify_token(token: &str, hash: &str) -> bool {
        hash_token(token) == hash
    }
}
// otpauth
// MFA secrets
pub mod tokens {
    use super::*;
    use base64::engine::general_purpose;
    use base64::Engine;
    use chacha20poly1305::aead::generic_array::typenum::Unsigned;
    use chacha20poly1305::aead::generic_array::GenericArray;
    use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
    use chacha20poly1305::ChaCha20Poly1305;
    use rand::distr::Alphanumeric;
    use rand::{rng, Rng, RngCore};
    use totp_rs::qrcodegen_image::image::EncodableLayout;
    use totp_rs::{Algorithm, Secret, TOTP};

    pub fn generate_token() -> String {
        //Gnerate cryptographic 256-bit secure token
        let mut bytes = [0u8; 32];
        rng().fill_bytes(&mut bytes);
        general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
    }

    pub fn generate_code(length: u8) -> String {
        let mut rng = rand::rng();
        let chars: String = (0..length)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();
        chars
    }

    pub fn generate_recovery_codes(count: u8) -> Vec<String> {
        (0..count).map(|_| generate_code(8)).collect()
    }

    pub fn generate_mfa_secret() -> String {
        Secret::generate_secret().to_string()
    }

    pub fn generate_qr_code(email: &str, secret: &str) -> Result<String> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.into(),
            Some(CONFIG.app_name.clone()),
            email.to_string(),
        )?;

        let qr_code = totp.get_qr_base64()?;
        Ok(format!("data:image/png;base64,{}", qr_code))
    }

    pub fn verify_totp(secret: &str, token: &str) -> Result<bool> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.into(),
            None,
            String::new(),
        )?;

        // This automatically handles time window tolerance
        Ok(totp.check_current(token)?)
    }

    pub fn encrypt(value: &str) -> Result<String> {
        // Key must be 32 bytes
        let key = CONFIG.mfa_secret_key.as_bytes();
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        // Encrypt value then insert nonce to start of value
        let mut obsf = cipher.encrypt(&nonce, value.as_bytes())?;
        obsf.splice(..0, nonce.iter().copied());

        // Base64  encode
        let encrypted = general_purpose::STANDARD.encode(&obsf);

        Ok(encrypted.to_owned())
    }

    pub fn decrypt(value: &str) -> Result<String> {
        type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;

        // Get key
        let key = CONFIG.mfa_secret_key.as_bytes();
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));

        // decode base64 encoded value
        let encrypted_data = general_purpose::STANDARD.decode(value)?;

        // get the nonce from the value
        let (nonce, ciphertext) = encrypted_data.as_bytes().split_at(NonceSize::to_usize());

        // decrypt
        let nonce = GenericArray::from_slice(nonce);
        let plaintext = cipher.decrypt(nonce, ciphertext)?;
        let decrypted = str::from_utf8(&plaintext)?;

        Ok(decrypted.to_string())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anyhow;
//     use base64::{engine::general_purpose, Engine};
//     use chrono::Utc;
//     use ctor::ctor;
//     use totp_rs::{Algorithm, TOTP};
//
//     #[ctor]
//     fn load_env() {
//         let _ = dotenvy::dotenv();
//     }
//
//     #[test]
//     fn test_encode_full_jwt() -> anyhow::Result<()> {
//         let user_id = 1;
//
//         let jwt = jwt::encode_jwt(user_id, true)?;
//         let claim = jwt::decode_jwt(&jwt)?;
//
//         assert_eq!(claim.sub, user_id);
//         assert!(claim.exp > Utc::now().timestamp() as usize);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_encode_partial_jwt() -> anyhow::Result<()> {
//         let user_id = 1;
//
//         let jwt = jwt::encode_jwt(user_id, false)?;
//         let claim = jwt::decode_jwt(&jwt)?;
//
//         assert_eq!(claim.sub, user_id);
//         assert!(claim.exp > Utc::now().timestamp() as usize);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_validate_token() -> anyhow::Result<()> {
//         let user_id = 10;
//
//         let jwt = jwt::encode_jwt(user_id, true)?;
//         let is_valid = jwt::validate_jwt(&jwt);
//
//         assert!(is_valid);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_hash_password() -> anyhow::Result<()> {
//         let password = "password";
//
//         let hash = hash::hash_password(password)?;
//         let is_verified = hash::verify_password(password, &hash)?;
//
//         assert!(is_verified);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_hash_password_invalid() -> anyhow::Result<()> {
//         let password = "password";
//
//         let hash = hash::hash_password(password)?;
//         let is_verified = hash::verify_password("invalid", &hash)?;
//
//         assert_eq!(is_verified, false);
//         Ok(())
//     }
//
//     #[test]
//     fn test_generate_token() -> anyhow::Result<()> {
//         let token = tokens::generate_token();
//
//         let is_url_safe = general_purpose::URL_SAFE_NO_PAD.decode(token).is_ok();
//         assert!(is_url_safe);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_hash_token() -> anyhow::Result<()> {
//         let token = tokens::generate_token();
//         let hash = hash::hash_token(&token);
//
//         assert!(hash::verify_token(&token, &hash));
//
//         Ok(())
//     }
//
//     #[test]
//     fn generate_code() -> anyhow::Result<()> {
//         let code = tokens::generate_code(6);
//         assert_eq!(code.len(), 6);
//         assert!(code.chars().all(char::is_alphanumeric));
//
//         let codes = tokens::generate_recovery_codes(10);
//         assert_eq!(codes.len(), 10);
//         assert!(codes.iter().all(|c| c.len() == 8));
//
//         Ok(())
//     }
//     #[test]
//     fn generate_qr_code() -> anyhow::Result<()> {
//         let email = "test@gmail.com";
//         let secret = tokens::generate_mfa_secret();
//         let _qr_code = tokens::generate_qr_code(email, &secret)?;
//         // println!("{:?}", qr_code);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_verify_totp() -> anyhow::Result<()> {
//         let secret = tokens::generate_mfa_secret();
//         let totp = TOTP::new(
//             Algorithm::SHA1,
//             6,
//             1,
//             30,
//             secret.clone().into(),
//             None,
//             String::new(),
//         )?;
//         let token = totp.generate_current()?;
//
//         let is_valid = tokens::verify_totp(&secret, &token)?;
//         assert!(is_valid);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_verify_totp_invalid() -> anyhow::Result<()> {
//         let secret = tokens::generate_mfa_secret();
//
//         let is_valid = tokens::verify_totp(&secret, "invlaid")?;
//         assert_eq!(is_valid, false);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_encrypt() -> anyhow::Result<()> {
//         let value = "testing".to_string();
//
//         let encrypted = tokens::encrypt(&value)?;
//         let decrypted = tokens::decrypt(&encrypted)?;
//
//         assert_eq!(value, decrypted);
//         Ok(())
//     }
// }
