pub mod contactinfo_controller;
pub mod location_controller;
pub mod notification_controller;
pub mod service_controller;

use mongodb::{
    error::Error,
    sync::{Client, Database},
};

pub const DB_CON_ERR: &str = "Couldn't connect to database";

pub struct MongoRepo {
    db: Database,
}

impl MongoRepo {
    pub fn init(db_name: String, uri: String) -> Result<Self, Error> {
        let client = Client::with_uri_str(uri)?;
        let db = client.database(&db_name);
        Ok(MongoRepo { db })
    }
}
