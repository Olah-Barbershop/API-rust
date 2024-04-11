mod controllers;
mod custom;
mod models;

use std::env;
extern crate dotenv;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;
use rocket::{
    http::{Method, Status},
    Request,
};
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::controllers::contactinfo_controller::get_contactinfo;
use crate::controllers::location_controller::get_locations;
use crate::controllers::notification_controller::get_notifications;
use crate::controllers::service_controller::get_services;
use crate::controllers::MongoRepo;
use custom::error::CustomError;

#[catch(default)]
fn default_catcher(status: Status, _: &Request) -> Option<CustomError> {
    Some(CustomError::Default(status))
}

#[launch]
fn rocket() -> _ {
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

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Method::Get].into_iter().map(From::from).collect())
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .register("/", catchers![default_catcher])
        .manage(db)
        .mount(
            "/",
            routes![
                get_services,
                get_notifications,
                get_contactinfo,
                get_locations
            ],
        )
}
