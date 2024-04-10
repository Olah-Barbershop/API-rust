pub mod contactinfo_controller;
pub mod location_controller;
pub mod notification_controller;
pub mod service_controller;

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::{
    contactinfo_model::Contactinfo, location_model::Location, notification_model::Notification,
    service_model::Service,
};
use mongodb::{
    bson::{doc, extjson::de::Error},
    options::FindOptions,
    sync::{Client, Collection, Database},
};

pub struct MongoRepo {
    db: Database,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("DATABASE_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("obs");
        MongoRepo { db }
    }

    pub fn get_locations(&self) -> Result<Vec<Location>, Error> {
        let col: Collection<Location> = self.db.collection("locations");
        let cursors = col
            .find(
                None,
                FindOptions::builder().projection(doc! {"_id":0}).build(),
            )
            .ok()
            .expect("Error fetching locations");
        let locations = cursors.map(|doc| doc.unwrap()).collect();
        Ok(locations)
    }

    pub fn get_contactinfo(&self) -> Result<Vec<Contactinfo>, Error> {
        let col: Collection<Contactinfo> = self.db.collection("contactinfo");
        let cursors = col
            .find(
                None,
                FindOptions::builder()
                    .projection(doc! {"_id":0})
                    .sort(doc! {"_id": 1})
                    .build(),
            )
            .ok()
            .expect("Error fetching contactinfo");
        let contactinfo = cursors.map(|doc| doc.unwrap()).collect();
        Ok(contactinfo)
    }

    pub fn get_services(&self) -> Result<Vec<Service>, Error> {
        let col: Collection<Service> = self.db.collection("services");
        let cursors = col
            .find(
                None,
                FindOptions::builder().projection(doc! {"_id":0}).build(),
            )
            .ok()
            .expect("Error fetching services");
        let services = cursors.map(|doc| doc.unwrap()).collect();
        Ok(services)
    }

    pub fn get_notifications(&self) -> Result<Vec<Notification>, Error> {
        let col: Collection<Notification> = self.db.collection("notifications");
        let cursors = col
            .find(None, None)
            .ok()
            .expect("Error fetching notifications");
        let notifications = cursors.map(|doc| doc.unwrap()).collect();
        Ok(notifications)
    }
}
