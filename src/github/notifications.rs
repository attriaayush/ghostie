use crate::github::client::{Github, NotificationError};

use serde::Deserialize;

type DateTime = chrono::DateTime<chrono::Utc>;

pub struct Notifications {
    github: Github,
}

impl Notifications {
    pub fn new(github: Github) -> Self {
        Self { github }
    }

    pub fn list(&self) -> NotificationsBuilder {
        NotificationsBuilder::new(&self.github)
    }
}

#[derive(serde::Serialize)]
pub struct NotificationsBuilder<'client> {
    #[serde(skip)]
    github: &'client Github,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DateTime>,
}

impl<'client> NotificationsBuilder<'client> {
    fn new(github: &'client Github) -> Self {
        Self {
            github,
            since: None,
            before: None,
        }
    }

    pub fn since(mut self, since: DateTime) -> Self {
        self.since = Some(since);
        self
    }

    pub fn before(mut self, before: DateTime) -> Self {
        self.before = Some(before);
        self
    }

    pub async fn fetch(self) -> Result<Vec<Notification>, NotificationError> {
        self.github
            .get::<Vec<Notification>, NotificationsBuilder>("notifications", Some(&self))
            .await
    }
}

#[derive(Debug, Deserialize)]
pub struct Notification {
    pub id: String,
    pub unread: bool,
    pub updated_at: String,
    pub last_read_at: Option<String>,
    pub reason: String,
    pub url: String,
    pub subject: Subject,
    pub repository: Repository,
}

#[derive(Debug, Deserialize)]
pub struct Subject {
    pub title: String,
    pub url: Option<String>,
    pub latest_comment_url: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
}
