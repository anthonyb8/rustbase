use crate::smtp::messages::Email;
use crate::Result;
use lettre::Transport;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport};

#[derive(Debug, Clone)]
pub struct EmailService {
    mailer: SmtpTransport,
}

impl EmailService {
    pub fn new(username: &str, password: &str, relay: &str) -> Result<Self> {
        let creds = Credentials::new(username.to_string(), password.to_string());
        let mailer = SmtpTransport::relay(relay)?
            .credentials(creds.clone())
            .build();

        Ok(EmailService { mailer: mailer })
    }

    pub fn send_email(&self, email: Email) -> Result<()> {
        let msg: Message = email.into();

        match self.mailer.send(&msg) {
            Ok(_) => Ok(()),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::smtp::messages::{mfa_code_body, reset_password_body, verify_email_body};
//     use ctor::ctor;
//     use lettre::message::header::ContentType;
//     use lettre::message::Mailbox;
//
//     #[ctor]
//     fn load_env() {
//         let _ = dotenvy::dotenv();
//     }
//
//     #[test]
//     #[ignore]
//     fn test_verification_email() -> anyhow::Result<()> {
//         let service = EmailService::new(username, password, relay)?;
//         let email = Email {
//             recipient: Mailbox::new(
//                 Some("Anthony".to_owned()),
//                 "anthonynbaxter@gmail.com".parse().unwrap(),
//             ),
//             sender: Mailbox::new(Some("Info".to_owned()), "info@worklog.ca".parse().unwrap()),
//             subject: String::from("Happy new year"),
//             header: ContentType::TEXT_HTML,
//             body: verify_email_body("https://www.worklog.ca")?,
//         };
//
//         let result = service.send_email(email)?;
//         assert_eq!(result, ());
//
//         Ok(())
//     }
//
//     #[test]
//     #[ignore]
//     fn test_reset_password() -> anyhow::Result<()> {
//         let service = EmailService::new(username, password, relay)?;
//         let email = Email {
//             recipient: Mailbox::new(
//                 Some("Anthony".to_owned()),
//                 "anthonynbaxter@gmail.com".parse().unwrap(),
//             ),
//             sender: Mailbox::new(Some("Info".to_owned()), "info@worklog.ca".parse().unwrap()),
//             subject: String::from("Happy new year"),
//             header: ContentType::TEXT_HTML,
//             body: reset_password_body("https://www.worklog.ca")?,
//         };
//
//         let result = service.send_email(email)?;
//         assert_eq!(result, ());
//
//         Ok(())
//     }
//     #[test]
//     #[ignore]
//     fn test_mfa_code() -> anyhow::Result<()> {
//         let service = EmailService::new(username, password, relay)?;
//         let email = Email {
//             recipient: Mailbox::new(
//                 Some("Anthony".to_owned()),
//                 "anthonynbaxter@gmail.com".parse().unwrap(),
//             ),
//             sender: Mailbox::new(Some("Info".to_owned()), "info@worklog.ca".parse().unwrap()),
//             subject: String::from("MFA Code"),
//             header: ContentType::TEXT_HTML,
//             body: mfa_code_body("123456")?,
//         };
//
//         let result = service.send_email(email)?;
//         assert_eq!(result, ());
//
//         Ok(())
//     }
// }
