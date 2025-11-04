use crate::Result;
use lettre::{
    message::{header::ContentType, Mailbox},
    Message,
};
use std::env;

#[derive(Debug)]
pub struct Email {
    pub recipient: Mailbox,
    pub sender: Mailbox,
    pub subject: String,
    pub header: ContentType,
    pub body: String,
}

impl Into<Message> for Email {
    fn into(self) -> Message {
        Message::builder()
            .from(self.sender)
            .to(self.recipient)
            .subject(self.subject)
            .header(self.header)
            .body(self.body)
            .unwrap()
    }
}

pub fn verify_email_body(verification_url: &str) -> Result<String> {
    let path = env::current_dir()?.join("src/smtp/templates/verify-email.html");
    let contents = std::fs::read_to_string(path)?;
    let body = contents.replace("{{VERIFICATION_URL}}", verification_url);
    Ok(body)
}

pub fn reset_password_body(reset_url: &str) -> Result<String> {
    let path = env::current_dir()?.join("src/smtp/templates/reset-pass.html");
    let contents = std::fs::read_to_string(path)?;
    let body = contents.replace("{{RESET_URL}}", reset_url);
    Ok(body)
}

pub fn mfa_code_body(mfa_code: &str) -> Result<String> {
    let path = env::current_dir()?.join("src/smtp/templates/mfa-code.html");
    let contents = std::fs::read_to_string(path)?;
    let body = contents.replace("{{VERIFICATION_CODE}}", mfa_code);
    Ok(body)
}
