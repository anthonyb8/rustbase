use serde::Deserialize;

#[derive(Deserialize)]
pub struct VerificationEmail {
    pub email: String,
}
