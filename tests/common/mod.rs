use notify_rs::{Config, NotifyError, Platform};
use once_cell::sync::Lazy;
use std::sync::Once;

static INIT: Once = Once::new();
static TEST_CONFIG: Lazy<TestConfig> = Lazy::new(|| {
    INIT.call_once(|| {
        dotenv::from_path("tests/.env").expect("Failed to load .env file");
    });
    
    TestConfig::new().expect("Failed to initialize test config")
});

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub slack_token: String,
    pub slack_channel: String,
    pub discord_token: String,
    pub discord_channel: String,
    pub telegram_token: String,
    pub telegram_channel: String,
    pub telegram_group_id: String,
    pub pushover_token: String,
    pub pushover_user: String,
}

impl TestConfig {
    pub fn new() -> Result<Self, NotifyError> {
        Ok(Self {
            slack_token: get_env_var("SLACK_BOT_TOKEN")?,
            slack_channel: get_env_var("SLACK_CHANNEL_ID")?,
            discord_token: get_env_var("DISCORD_BOT_TOKEN")?,
            discord_channel: get_env_var("DISCORD_CHANNEL_ID")?,
            telegram_token: get_env_var("TELEGRAM_BOT_TOKEN")?,
            telegram_channel: get_env_var("TELEGRAM_CHAT_ID")?,
            telegram_group_id: get_env_var("TELEGRAM_GROUP_ID")?,
            pushover_token: get_env_var("PUSHOVER_APP_TOKEN")?,
            pushover_user: get_env_var("PUSHOVER_USER_KEY")?,
        })
    }

    pub fn get() -> &'static Self {
        &TEST_CONFIG
    }

    pub fn get_slack_config(&self) -> Config {
        Config {
            platform: Platform::Slack,
            token: self.slack_token.clone(),
            channel: self.slack_channel.clone(),
        }
    }

    pub fn get_discord_config(&self) -> Config {
        Config {
            platform: Platform::Discord,
            token: self.discord_token.clone(),
            channel: self.discord_channel.clone(),
        }
    }

    pub fn get_telegram_config(&self) -> Config {
        Config {
            platform: Platform::Telegram,
            token: self.telegram_token.clone(),
            channel: self.telegram_channel.clone(),
        }
    }

    pub fn get_telegram_group_config(&self) -> Config {
        Config {
            platform: Platform::Telegram,
            token: self.telegram_token.clone(),
            channel: self.telegram_group_id.clone(),
        }
    }

    pub fn get_pushover_config(&self) -> Config {
        Config {
            platform: Platform::Pushover,
            token: self.pushover_token.clone(),
            channel: self.pushover_user.clone(),
        }
    }
}

fn get_env_var(key: &str) -> Result<String, NotifyError> {
    std::env::var(key).map_err(|_| NotifyError::InvalidConfig(format!("{} not set in environment", key)))
}
