use crate::cache::{Cache, notifications::Notification};

pub fn read_all_notifications() -> Vec<Notification> {
    Cache::new().read_all().unwrap()
}

pub fn mark_as_read(id: &str) {
    Cache::new().delete_by_id(id).unwrap();
}
