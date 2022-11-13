use std::{collections::HashMap, time::Duration};

use clokwerk::*;

use crate::{
    cache::{notifications::Notification, Cache},
    config::github_token,
    github::client::{Credentials, Github},
    platform,
};

use crate::{error, info};

async fn fetch_notifications() -> Vec<Notification> {
    let notifications: Vec<Notification> = Github::init_with_token(Credentials::Token(github_token()))
        .user_activity()
        .notifications()
        .list()
        .since(chrono::Utc::now() - chrono::Duration::days(2))
        .fetch()
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
    let cache = Cache::new();

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
        platform::notification::send(&format!("{} new notifications", count), Duration::from_secs(3));
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
