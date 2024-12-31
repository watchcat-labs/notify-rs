mod common;
use common::TestConfig;
use notify_rs::Notify;

#[tokio::test]
async fn test_slack_notify() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig::get().get_slack_config();
    println!("Testing Slack notification...");
    let notify = Notify::new(config)?;
    notify.send("test message from notify-rs (slack)").await?;
    println!("Slack test completed");
    Ok(())
}

#[tokio::test]
async fn test_discord_notify() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig::get().get_discord_config();
    println!("Testing Discord notification...");
    let notify = Notify::new(config)?;
    notify.send("test message from notify-rs (discord)").await?;
    println!("Discord test completed");
    Ok(())
}

#[tokio::test]
async fn test_telegram_notify() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig::get().get_telegram_config();
    println!("Testing Telegram notification...");
    let notify = Notify::new(config)?;
    notify.send("test message from notify-rs (telegram)").await?;
    println!("Telegram test completed");
    Ok(())
}

#[tokio::test]
async fn test_telegram_notify_group() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig::get().get_telegram_group_config();
    println!("Testing Telegram group notification...");
    let notify = Notify::new(config)?;
    notify.send("test message from notify-rs (telegram group)").await?;
    println!("Telegram group test completed");
    Ok(())
}

#[tokio::test]
async fn test_pushover_notify() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig::get().get_pushover_config();
    println!("Testing Pushover notification...");
    let notify = Notify::new(config)?;
    notify.send("test message from notify-rs (pushover)").await?;
    println!("Pushover test completed");
    Ok(())
}
