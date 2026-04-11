use std::{collections::HashSet, time::Duration};

use tracing::{error, info};

use crate::{
    cache::{Cache, notifications::Notification},
    configuration::{config::Config, token::Token},
    github::client::{Credentials, Github},
    platform,
};

fn github_instance(token: &str) -> Github {
    Github::init_with_token(Credentials::Token(token.to_string()))
}

fn rolling_window() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now() - chrono::Duration::days(Config::read().additional_config.polling_window_days.into())
}

pub async fn mark_notification_as_read(notifcation_id: &str) {
    let token = Token::get();
    if let Err(e) = github_instance(&token)
        .user_activity()
        .notifications()
        .builder()
        .mark_as_read(notifcation_id)
        .await
    {
        error!("Failed to mark notification as read: {}", e);
    }
}

async fn fetch_notifications(token: &str) -> Option<Vec<Notification>> {
    match github_instance(token)
        .user_activity()
        .notifications()
        .builder()
        .since(rolling_window())
        .list()
        .await
    {
        Ok(notifications) => Some(notifications.into_iter().map(|n| n.into()).collect()),
        Err(e) => {
            error!("Failed to fetch notifications: {}", e);
            None
        }
    }
}

async fn poll_notifications(token: &str) {
    let mut cache = Cache::new();
    cache.delete_all_before(rolling_window());

    let cached_ids: HashSet<String> = cache.read_all().unwrap_or_default().into_iter().map(|n| n.id).collect();

    let Some(fetched) = fetch_notifications(token).await else {
        return;
    };

    let notifications: Vec<_> = fetched
        .into_iter()
        .filter(|notification| !cached_ids.contains(&notification.id))
        .collect();

    if let Err(e) = cache.write_batch(&notifications) {
        error!("Failed to write to the cache: {}", e);
        return;
    }

    let count = notifications.len();
    if count > 0 && Config::read().additional_config.enable_os_notifications {
        let notification = platform::notification::NotificationManager::new();
        notification.send(format!("{} new notifications", count), Duration::from_secs(3));
    }

    info!("Found {} new notifications", count);
}

pub async fn start() {
    let token = Token::get();
    let secs = Config::read().additional_config.polling_interval_seconds;
    let mut interval = tokio::time::interval(Duration::from_secs(secs.into()));

    loop {
        interval.tick().await;
        poll_notifications(&token).await;
    }
}
