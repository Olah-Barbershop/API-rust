use crate::{controllers::MongoRepo, custom::error::CustomError, models::location_model::Location};
use rocket::{http::Status, serde::json::Json, State};

#[get("/locations")]
pub fn get_locations(db: &State<MongoRepo>) -> Result<Json<Vec<Location>>, CustomError> {
    let locations = db.get_locations();
    match locations {
        Ok(locations) => {
            if locations.is_empty() {
                Err(CustomError::Default(Status::NotFound))
            } else {
                Ok(Json(locations))
            }
        }
        Err(_) => Err(CustomError::Default(Status::InternalServerError)),
    }
}
