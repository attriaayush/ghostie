use rusqlite::{Connection, Result};

use crate::cache::database::Database;
use crate::github::notifications::Notification as GithubNotification;

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub name: String,
    pub repo: String,
    pub subject: String,
    pub kind: String,
    pub url: String,
    pub updated_at: String,
}

fn create_github_url(repo_url: String, subject_url: Option<String>) -> String {
    if subject_url.is_none() {
        return repo_url;
    }

    let mut html_url = subject_url.unwrap().replace("api.github.com/repos", "github.com");

    if html_url.contains("/pulls/") {
        html_url = html_url.replace("/pulls/", "/pull/");
    }

    html_url
}

impl From<GithubNotification> for Notification {
    fn from(github_notification: GithubNotification) -> Notification {
        Notification {
            id: github_notification.id,
            name: github_notification.repository.full_name,
            subject: github_notification.subject.title,
            repo: github_notification.repository.name,
            kind: github_notification.subject.kind,
            url: create_github_url(github_notification.repository.html_url, github_notification.subject.url),
            updated_at: github_notification.updated_at,
        }
    }
}

pub struct Cache {
    pub instance: Connection,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        let db_client = Database::create().unwrap();
        Self {
            instance: db_client.connection,
        }
    }

    pub fn destroy() -> Result<Self> {
        let db_client = Database::destroy().unwrap();
        Ok(Self {
            instance: db_client.connection,
        })
    }

    pub fn delete_all_before(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        let notifications = self.read_all().unwrap();
        notifications
            .iter()
            .filter(|n| chrono::DateTime::parse_from_rfc2822(&n.updated_at).unwrap() < timestamp)
            .for_each(|notification| self.delete_by_id(&notification.id).unwrap());
    }

    pub fn delete_all(&mut self) -> Result<()> {
        self.instance.execute("DELETE FROM ghostie", [])?;
        Ok(())
    }

    pub fn delete_by_id(&self, id: &str) -> Result<()> {
        self.instance
            .execute("DELETE FROM ghostie WHERE id = :id", &[(":id", &id.to_owned())])?;

        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<Notification>> {
        let mut statement = self.instance.prepare("SELECT * FROM ghostie")?;
        let notification_iter = statement.query_map([], |row| {
            Ok(Notification {
                id: row.get(0)?,
                name: row.get(1)?,
                repo: row.get(2)?,
                subject: row.get(3)?,
                kind: row.get(4)?,
                url: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut notifications = Vec::<Notification>::new();
        for notification in notification_iter {
            notifications.push(notification?)
        }

        Ok(notifications)
    }

    pub fn read_by_id(&self, id: &str) -> Result<Notification> {
        let notification = self
            .instance
            .query_row("SELECT * FROM ghostie WHERE id = :id", &[(":id", id)], |row| {
                Ok(Notification {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    repo: row.get(2)?,
                    subject: row.get(3)?,
                    kind: row.get(4)?,
                    url: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?;

        Ok(notification)
    }

    pub fn write(&self, notification: &Notification) -> Result<()> {
        self.instance.execute(
            "INSERT OR REPLACE INTO ghostie (id, name, repo, subject, kind, url, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
              notification.id.to_owned(),
              notification.name.to_owned(),
              notification.repo.to_owned(),
              notification.subject.to_owned(),
              notification.kind.to_owned(),
              notification.url.to_owned(),
              notification.updated_at.to_owned()),
        )?;

        Ok(())
    }

    pub fn write_batch(&self, notifications: &[Notification]) -> Result<()> {
        for notification in notifications.iter() {
            Self::write(self, notification)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Cache, Notification};
    use fake::{Fake as Generate, Faker};
    use serial_test::serial;

    struct Fake(Notification);

    fn clear_cache() {
        Cache::destroy().unwrap();
    }

    impl Fake {
        fn a_notification(id: String) -> Notification {
            Notification {
                id,
                name: String::from("Fake Notification"),
                repo: String::from("Fake repo"),
                kind: String::from("pull_request"),
                subject: String::from("I need review"),
                url: String::from("https://github.com/"),
                updated_at: String::from("Mon, 21 Nov 2022 10:59:42 +0000"),
            }
        }

        fn list_of_notifications(count: usize) -> Vec<Notification> {
            let mut notifications = Vec::with_capacity(count);

            for _ in 0..count {
                let random_string: String = Faker.fake();
                notifications.push(Fake::a_notification(random_string));
            }
            notifications
        }
    }

    const _ID: &str = "12";

    #[test]
    #[serial]
    fn delete_by_timestamp() {
        clear_cache();
        let mut instance = Cache::new();
        instance.write(&Fake::a_notification(_ID.to_owned())).unwrap();

        instance.delete_all_before(chrono::offset::Utc::now());

        let notification = instance.read_by_id(_ID).is_ok();
        println!("{}", notification);
    }

    #[test]
    #[serial]
    fn write_and_read() {
        clear_cache();
        let instance = Cache::new();
        let write = instance.write(&Fake::a_notification(_ID.to_owned())).is_ok();

        assert!(write);

        let notification = instance.read_by_id(_ID).unwrap();
        assert_eq!(_ID, notification.id);
    }

    #[test]
    #[serial]
    fn write_and_delete() {
        clear_cache();
        let instance = Cache::new();
        instance.write(&Fake::a_notification(_ID.to_owned())).unwrap();

        let deleted = instance.delete_by_id(_ID).is_ok();
        assert!(deleted);

        let notification = instance.read_by_id(_ID).is_ok();
        assert!(!notification);
    }

    #[test]
    #[serial]
    fn batch_write_and_read() {
        clear_cache();
        let count = 5;
        let instance = Cache::new();
        let write = instance.write_batch(&Fake::list_of_notifications(count)).is_ok();
        assert!(write);

        let notifications = instance.read_all().unwrap();
        assert_eq!(count, notifications.len());
    }
}
