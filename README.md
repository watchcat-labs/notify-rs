# notify-rs

A simple, flexible notification library for Rust that supports multiple platforms.

## Features

- Support multiple notification platforms:
  - Telegram
  - Discord
  - Slack
  - Pushover

- Easy to use API
- Async support
- Configurable through environment variables
- Error handling with custom error types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
notify-rs = "0.1.0"
```

## Usage

### Basic Example

```rust
use notify_rs::{Config, Notify, Platform};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init configuration
    let config = Config {
        platform: Platform::Telegram,
        token: "YOUR_BOT_TOKEN".to_string(),
        channel: "CHAT_ID".to_string(),
    };

    // Create notifier
    let notify = Notify::new(config)?;

    // Send notification
    notify.send("Hello from notify-rs!").await?;

    Ok(())
}
```

### Using Environment Variables

```rust
use notify_rs::{Config, Notify, Platform};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    let config = Config {
        platform: Platform::Telegram,
        token: env::var("TELEGRAM_BOT_TOKEN")?,
        channel: env::var("TELEGRAM_CHAT_ID")?,
    };

    let notify = Notify::new(config)?;
    notify.send("Message from environment config!").await?;

    Ok(())
}
```

## Platform-specific Configuration

### Telegram

```rust
let config = Config {
    platform: Platform::Telegram,
    token: "BOT_TOKEN".to_string(),    // Bot token from @BotFather
    channel: "CHAT_ID".to_string(),    // Numeric chat ID
};
```

- To get bot token: Talk to [@BotFather](https://t.me/botfather)
- To get chat ID: Send a message to [@get_id_bot](https://t.me/get_id_bot)

### Discord

```rust
let config = Config {
    platform: Platform::Discord,
    token: "BOT_TOKEN".to_string(),    // Bot token
    channel: "CHANNEL_ID".to_string(), // Channel ID
};
```

### Slack

```rust
let config = Config {
    platform: Platform::Slack,
    token: "BOT_TOKEN".to_string(),    // Bot User OAuth Token
    channel: "CHANNEL_ID".to_string(), // Channel ID
};
```

### Pushover

```rust
let config = Config {
    platform: Platform::Pushover,
    token: "APP_TOKEN".to_string(),    // Application token
    channel: "USER_KEY".to_string(),   // User key
};
```

## Development

### Running Tests

Create a `tests/.env` file with your test credentials:

```bash
cp tests/.env.example tests/.env
```

Then edit `tests/.env` with your actual tokens and IDs.

Run the tests:
```bash
cargo test
```

### Environment Variables

Create a `.env` file:

```env
# Telegram
TELEGRAM_BOT_TOKEN=your_bot_token
TELEGRAM_CHAT_ID=your_chat_id

# Discord
DISCORD_BOT_TOKEN=your_bot_token
DISCORD_CHANNEL_ID=your_channel_id

# Slack
SLACK_BOT_TOKEN=your_bot_token
SLACK_CHANNEL_ID=your_channel_id

# Pushover
PUSHOVER_APP_TOKEN=your_app_token
PUSHOVER_USER_KEY=your_user_key
```
