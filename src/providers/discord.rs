#![allow(unused)]

use crate::{NotifyError, NotifyProvider};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://discord.com/api/webhooks/";

#[derive(Debug, Serialize)]
struct WebhookMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct DiscordResponse {
    ok: Option<bool>,
    error: Option<String>,
}

pub struct DiscordProvider {
    token: String,
    channel: String,
    client: reqwest::Client,
}

impl DiscordProvider {
    pub fn new(token: String, channel: String) -> Self {
        Self {
            token,
            channel,
            client: reqwest::Client::new(),
        }
    }

    fn get_webhook_url(&self) -> String {
        format!("{}{}/{}", API_URL, self.channel, self.token)
    }
}

#[async_trait]
impl NotifyProvider for DiscordProvider {
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

        let webhook_msg = WebhookMessage {
            content: message.to_string(),
        };

        let response = self.client
            .post(&self.get_webhook_url())
            .json(&webhook_msg)
            .send()
            .await
            .map_err(|e| NotifyError::RequestFailed(e.to_string()))?;

        if response.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(());
        }

        if let Ok(error_response) = response.json::<DiscordResponse>().await {
            if let Some(error) = error_response.error {
                return Err(NotifyError::RequestFailed(error));
            }
        }

        Err(NotifyError::RequestFailed("Unknown error".to_string()))
    }
}
