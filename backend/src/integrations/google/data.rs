use base64::{engine::general_purpose, Engine as _};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct MessageQuery {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GmailList {
    messages: Vec<GmailMessage>,

    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,

    #[serde(rename = "resultSizeEstimate")]
    result_size_estimate: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GmailMessage {
    id: String,

    #[serde(rename = "threadId")]
    thread_id: String,
}

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub id: String,
    pub from: String,
    pub delivered_to: String,
    pub subject: String,
    pub snippet: String,
    pub body: String,
    // pub email_account_id: String,
    // pub message_id: String,
    // pub from_email: String,
    // pub from_name: Option<String>,
    // pub to_emails: Vec<String>,
    // pub cc_emails: Option<Vec<String>>,
    // pub bcc_emails: Option<Vec<String>>,
    // pub body_html: Option<String>,
    // pub is_read: bool,
    // pub is_important: bool,
    // pub is_pinned: bool,
    // pub has_attachments: bool,
    // pub received_at: DateTime<Utc>,
    // pub sent_at: Option<DateTime<Utc>>,
    // pub labels: Option<Vec<String>>,
    // pub athlete_ids: Option<Vec<String>>,
    // pub contact_ids: Option<Vec<String>>,
    // pub deal_ids: Option<Vec<String>>,
    // pub created_at: DateTime<Utc>,
}

impl<'de> Deserialize<'de> for EmailMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = Value::deserialize(deserializer)?;
        let id = json["id"].as_str().unwrap_or("").to_string();
        let snippet = get_snippet(&json);
        let headers = json["payload"]["headers"]
            .as_array()
            .expect("Should be headers.");

        let subject = get_header(headers, "Subject");
        let from = get_header(headers, "From");
        let delivered_to = get_header(headers, "Delivered-To");
        let body = get_body(&json).unwrap_or("".into()).to_string();
        println!("{:?}", id);
        Ok(EmailMessage {
            id,
            from,
            delivered_to,
            subject,
            snippet,
            body,
        })
    }
}

fn get_snippet(json: &Value) -> String {
    let snippet = json["snippet"].as_str().unwrap_or("").to_string();

    snippet
        .split('…')
        .next()
        .unwrap_or(&snippet)
        .trim()
        .to_string()
}

fn get_header(headers: &[Value], name: &str) -> String {
    headers
        .iter()
        .find(|h| h["name"] == name)
        .and_then(|h| h["value"].as_str())
        .unwrap_or("")
        .to_string()
}

fn get_body(json: &Value) -> Option<String> {
    let encoded = json
        .get("payload")?
        .get("parts")?
        .as_array()?
        .first()?
        .get("body")?
        .get("data")?
        .as_str()?;

    // Decode using the URL-safe alphabet
    let decoded_bytes = general_purpose::URL_SAFE
        .decode(encoded)
        .expect("invalid base64 data");

    String::from_utf8(decoded_bytes).ok()
}
