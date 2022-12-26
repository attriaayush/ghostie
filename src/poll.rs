use std::{collections::HashMap, time::Duration};

use clokwerk::*;

use crate::{
    cache::{notifications::Notification, Cache},
    config::github_token,
    github::client::{Credentials, Github},
    platform,
};

use crate::{error, info};

fn github_instance() -> Github {
    Github::init_with_token(Credentials::Token(github_token()))
}

fn rolling_window() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now() - chrono::Duration::days(2)
}

pub async fn mark_notification_as_read(notifcation_id: &str) {
    github_instance()
        .user_activity()
        .notifications()
        .builder()
        .mark_as_read(notifcation_id)
        .await
        .unwrap();
}

async fn fetch_notifications() -> Vec<Notification> {
    let notifications: Vec<Notification> = github_instance()
        .user_activity()
        .notifications()
        .builder()
        .since(rolling_window())
        .list()
        .await
        .unwrap_or_else(|error| {
            error!("\n");
            panic!("Failed to fetch notifications, {}", error)
        })
        .into_iter()
        .map(|notification| notification.into())
        .collect();

    notifications
}

async fn poll_notifications() {
    let mut cache = Cache::new();
    cache.delete_all_before(rolling_window());

    let mut cached_notifications_map = HashMap::new();
    for notification in cache.read_all().unwrap().iter() {
        cached_notifications_map.insert(notification.id.clone(), true);
    }

    let notifications: Vec<_> = fetch_notifications()
        .await
        .into_iter()
        .filter(|notification| !cached_notifications_map.contains_key(&notification.id))
        .collect();

    cache.write_batch(&notifications).unwrap_or_else(|error| {
        error!("\n");
        panic!("Failed to write to the cache, {}", error)
    });

    let count = notifications.len();
    if count > 0 {
        let notification = platform::notification::NotificationManager::new();
        notification.send(format!("{} new notifications", count), Duration::from_secs(3));
    }

    info!(format!("Found {} new notifications", count));
}

pub async fn start() {
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(1.minutes()).run(|| async {
        poll_notifications().await;
    });

    loop {
        scheduler.run_pending().await;
        async_std::task::sleep(Duration::from_millis(100)).await;
    }
}
