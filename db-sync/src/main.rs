#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel::connection::Connection;
use failure::Error;
use log::debug;
use serde_derive::Deserialize;
use diesel_migrations::embed_migrations;

embed_migrations!();

#[derive(Deserialize)]
struct Config {
    database: Option<String>,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let mut config = config::Config::default();
    config.merge(config::Environment::with_prefix("DBSYNC"))?;
    let config: Config = config.try_into()?;
    let db_address = config.database.unwrap_or("postgres://localhost/voting".into());
    debug!("Waiting for database...");
    loop {
        let conn: Result<PgConnection, _> = Connection::establish(&db_address);
        if let Ok(conn) = conn {
            debug!("Database connected");
            embedded_migrations::run(&conn)?;
            break;
        }
    }
    debug!("Database migrated");
    Ok(())
}
