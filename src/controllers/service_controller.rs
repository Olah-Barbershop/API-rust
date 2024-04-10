use crate::{controllers::MongoRepo, custom::error::CustomError, models::service_model::Service};
use rocket::{http::Status, serde::json::Json, State};

#[get("/services")]
pub fn get_services(db: &State<MongoRepo>) -> Result<Json<Vec<Service>>, CustomError> {
    let services = db.get_services();
    match services {
        Ok(services) => {
            if services.is_empty() {
                Err(CustomError::Default(Status::NotFound))
            } else {
                Ok(Json(services))
            }
        }
        Err(_) => Err(CustomError::Default(Status::InternalServerError)),
    }
}
