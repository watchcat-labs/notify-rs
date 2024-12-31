use serde::{Deserialize, Serialize};
use thiserror::Error;

mod providers;
pub use providers::*;

#[derive(Debug, Error)]
pub enum NotifyError {
    #[error("Platform not supported")]
    UnsupportedPlatform,
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Slack,
    Telegram,
    Discord,
    Pushover,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub platform: Platform,
    pub token: String,
    pub channel: String,
}

#[async_trait::async_trait]
pub trait NotifyProvider: Send + Sync {
    async fn send(&self, message: &str) -> Result<(), NotifyError>;
}

pub struct Notify {
    provider: Box<dyn NotifyProvider>,
}

impl Notify {
    pub fn new(config: Config) -> Result<Self, NotifyError> {
        let provider: Box<dyn NotifyProvider> = match config.platform {
            Platform::Telegram => Box::new(providers::telegram::TelegramProvider::new(
                config.token,
                config.channel,
            )),
            Platform::Slack => Box::new(providers::slack::SlackProvider::new(
                config.token,
                config.channel,
            )),
            Platform::Discord => Box::new(providers::discord::DiscordProvider::new(
                config.token,
                config.channel,
            )),
            Platform::Pushover => Box::new(providers::pushover::PushoverProvider::new(
                config.token,
                config.channel,
            )),
        };

        Ok(Self { provider })
    }

    pub async fn send(&self, message: &str) -> Result<(), NotifyError> {
        self.provider.send(message).await
    }
}
