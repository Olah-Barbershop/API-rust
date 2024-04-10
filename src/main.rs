mod controllers;
mod custom;
mod models;

#[macro_use]
extern crate rocket;
use rocket::{http::Status, Request};

use crate::controllers::contactinfo_controller::get_contactinfo;
use crate::controllers::location_controller::get_locations;
use crate::controllers::notification_controller::get_notifications;
use crate::controllers::service_controller::get_services;
use crate::controllers::MongoRepo;
use custom::error::CustomError;

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> Option<CustomError> {
    Some(CustomError::Default(status))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
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
