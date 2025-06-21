use diesel::{Connection, ConnectionError, PgConnection};

use crate::config::Config;

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn = PgConnection::establish(&config.database_url)?;

        Ok(Self { conn })
    }
}
