use crate::{NotifyError, NotifyProvider};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://slack.com/api/chat.postMessage";

#[derive(Debug, Serialize)]
struct SlackRequest {
    token: String,
    channel: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct SlackResponse {
    ok: bool,
    error: Option<String>,
}

pub struct SlackProvider {
    token: String,
    channel: String,
    client: reqwest::Client,
}

impl SlackProvider {
    pub fn new(token: String, channel: String) -> Self {
        Self {
            token,
            channel,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NotifyProvider for SlackProvider {
    async fn send(&self, message: &str) -> Result<(), NotifyError> {
        if self.token.is_empty() {
            return Err(NotifyError::MissingField("token".to_string()));
        }
        if self.channel.is_empty() {
            return Err(NotifyError::MissingField("channel".to_string()));
        }
        if message.is_empty() {
            return Err(NotifyError::MissingField("message".to_string()));
        }

        let request = SlackRequest {
            token: self.token.clone(),
            channel: self.channel.clone(),
            text: message.to_string(),
        };

        let response = self.client
            .post(API_URL)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request)
            .send()
            .await
            .map_err(|e| NotifyError::RequestFailed(e.to_string()))?;

        let result: SlackResponse = response
            .json()
            .await
            .map_err(|e| NotifyError::RequestFailed(e.to_string()))?;

        if !result.ok {
            return Err(NotifyError::RequestFailed(
                result.error.unwrap_or_else(|| "Unknown error".to_string())
            ));
        }

        Ok(())
    }
}
