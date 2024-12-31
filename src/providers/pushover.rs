use crate::{NotifyError, NotifyProvider};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.pushover.net/1/messages.json";

#[derive(Debug, Serialize)]
struct PushoverRequest {
    token: String,
    user: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct PushoverResponse {
    status: i32,
    #[serde(default)]
    errors: Vec<String>,
}

pub struct PushoverProvider {
    token: String,
    user: String,
    client: reqwest::Client,
}

impl PushoverProvider {
    pub fn new(token: String, user: String) -> Self {
        Self {
            token,
            user,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NotifyProvider for PushoverProvider {
    async fn send(&self, message: &str) -> Result<(), NotifyError> {
        if self.token.is_empty() {
            return Err(NotifyError::MissingField("token".to_string()));
        }
        if self.user.is_empty() {
            return Err(NotifyError::MissingField("user".to_string()));
        }
        if message.is_empty() {
            return Err(NotifyError::MissingField("message".to_string()));
        }

        let request = PushoverRequest {
            token: self.token.clone(),
            user: self.user.clone(),
            message: message.to_string(),
        };

        let response = self.client
            .post(API_URL)
            .json(&request)
            .send()
            .await
            .map_err(|e| NotifyError::RequestFailed(e.to_string()))?;

        let result: PushoverResponse = response
            .json()
            .await
            .map_err(|e| NotifyError::RequestFailed(e.to_string()))?;

        if result.status != 1 {
            return Err(NotifyError::RequestFailed(
                result.errors.first()
                    .cloned()
                    .unwrap_or_else(|| "Unknown error".to_string())
            ));
        }

        Ok(())
    }
}
