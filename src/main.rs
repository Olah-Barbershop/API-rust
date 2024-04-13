mod controllers;
mod custom;
mod models;

use std::env;
extern crate dotenv;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    Request, Response,
};

use crate::controllers::contactinfo_controller::get_contactinfo;
use crate::controllers::location_controller::get_locations;
use crate::controllers::notification_controller::{get_all_notifications, get_notification};
use crate::controllers::service_controller::get_services;
use crate::controllers::MongoRepo;
use custom::error::CustomError;

#[catch(default)]
fn default_catcher(status: Status, _: &Request) -> Option<CustomError> {
    Some(CustomError::Default(status))
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let uri = match env::var("DATABASE_URL") {
        Ok(v) => v.to_string(),
        Err(_) => panic!("Couldn't load env variable: DATABASE_URL"),
    };

    let db_name = match env::var("DATABASE_NAME") {
        Ok(v) => v.to_string(),
        Err(_) => panic!("Couldn't load env variable: DATABASE_NAME"),
    };

    let db = match MongoRepo::init(db_name, uri) {
        Ok(db) => db,
        Err(_) => panic!("Couldn't connect to database"),
    };

    pub struct CORS();

    #[rocket::async_trait]
    impl Fairing for CORS {
        fn info(&self) -> Info {
            Info {
                name: "Add CORS headers to requests",
                kind: Kind::Response,
            }
        }

        async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, PATCH, OPTIONS",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
    }

    rocket::build()
        .register("/", catchers![default_catcher])
        .manage(db)
        .attach(CORS())
        .mount(
            "/",
            routes![
                get_services,
                get_notification,
                get_all_notifications,
                get_contactinfo,
                get_locations
            ],
        )
}
