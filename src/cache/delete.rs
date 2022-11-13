use crate::cache::Cache;

pub fn prune_all() {
    Cache::new().delete_all().unwrap();
}
