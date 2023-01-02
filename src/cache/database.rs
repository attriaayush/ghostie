use anyhow::Result;
use rusqlite::Connection;

use crate::configuration::config::Config;
extern crate dirs;

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn create() -> Result<Self> {
        let connection = Self::create_connection();
        Self::migrate_up(&connection);

        Ok(Self { connection })
    }

    pub fn destroy() -> Result<Self> {
        let connection = Self::create_connection();
        Self::migrate_down(&connection);

        Ok(Self { connection })
    }

    fn create_connection() -> Connection {
        let db_file = &Config::read().cache_file;
        Connection::open(db_file)
            .unwrap_or_else(|err| panic!("There was an error creating the database connection, error: {}", err))
    }

    fn migrate_down(connection: &Connection) {
        connection
            .execute("DROP TABLE IF EXISTS ghostie", ())
            .unwrap_or_else(|err| panic!("There was an error dropping the database, error: {}", err));
    }

    fn migrate_up(connection: &Connection) {
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS ghostie(
                  id TEXT PRIMARY KEY,
                  name TEXT NOT NULL,
                  repo TEXT NOT NULL,
                  subject TEXT NOT NULL,
                  kind TEXT NOT NULL,
                  url TEXT NOT NULL,
                  updated_at TEXT NOT NULL
              )",
                (),
            )
            .unwrap_or_else(|err| panic!("There was an error migrating up. Error: {}", err));
    }
}
