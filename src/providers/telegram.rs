#![allow(unused)]

use crate::{NotifyError, NotifyProvider};
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

const API_URL: &str = "https://api.telegram.org/bot";

#[derive(Debug, Deserialize)]
struct Response {
    ok: bool,
    result: Option<Message>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Message {
    message_id: i64,
}

pub struct TelegramProvider {
    token: String, // 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
    chat_id: i64,
    client: reqwest::Client,
}

/// tg docs: https://core.telegram.org/bots/api#sendmessage

impl TelegramProvider {
    pub fn new(token: String, chat_id: String) -> Self {
        let _chat_id = chat_id.parse::<i64>().unwrap();
        Self {
            token,
            chat_id: _chat_id,
            client: reqwest::Client::new(),
        }
    }

    fn get_api_url(&self, method: &str) -> String {
        format!("{}{}/{}", API_URL, self.token, method)
    }
}

#[async_trait]
impl NotifyProvider for TelegramProvider {
    async fn send(&self, message: &str) -> Result<(), NotifyError> {
        if self.token.is_empty() {
            return Err(NotifyError::MissingField("token".to_string()));
        }

        if self.chat_id == 0 {
            return Err(NotifyError::MissingField("chat_id".to_string()));
        }

        if message.is_empty() {
            return Err(NotifyError::MissingField("message".to_string()));
        }

        let api_url = self.get_api_url("sendMessage");

        let params = serde_json::json!({
            "chat_id": &self.chat_id,
            "text": message,
        });
        println!(
            "Serialized params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        let response = self
            .client
            .post(&api_url)
            .json(&params)
            .send()
            .await
            .map_err(|e| NotifyError::RequestFailed(e.to_string()))?;

        let result: Response = response
            .json()
            .await
            .map_err(|e| NotifyError::RequestFailed(e.to_string()))?;

        if !result.ok {
            return Err(NotifyError::RequestFailed(
                result
                    .description
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        Ok(())
    }
}
