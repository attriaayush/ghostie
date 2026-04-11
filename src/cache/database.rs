use anyhow::Result;
use rusqlite::Connection;

use crate::configuration::config::Config;

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn create() -> Result<Self> {
        let connection = Self::create_connection()?;
        Self::migrate_up(&connection)?;

        Ok(Self { connection })
    }

    pub fn destroy() -> Result<Self> {
        let connection = Self::create_connection()?;
        Self::migrate_down(&connection)?;

        Ok(Self { connection })
    }

    fn create_connection() -> Result<Connection> {
        let db_file = &Config::read().cache_file;
        let connection = Connection::open(db_file)?;
        Ok(connection)
    }

    fn migrate_down(connection: &Connection) -> Result<()> {
        connection.execute("DROP TABLE IF EXISTS ghostie", ())?;
        Ok(())
    }

    fn migrate_up(connection: &Connection) -> Result<()> {
        connection.execute(
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
        )?;
        Ok(())
    }
}
