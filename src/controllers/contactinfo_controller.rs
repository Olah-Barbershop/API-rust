use crate::{
    controllers::MongoRepo, custom::error::CustomError, models::contactinfo_model::Contactinfo,
};
use rocket::{http::Status, serde::json::Json, State};

#[get("/contactinfo")]
pub fn get_contactinfo(db: &State<MongoRepo>) -> Result<Json<Vec<Contactinfo>>, CustomError> {
    let contactinfo = db.get_contactinfo();
    match contactinfo {
        Ok(contactinfo) => {
            if contactinfo.is_empty() {
                Err(CustomError::Default(Status::NotFound))
            } else {
                Ok(Json(contactinfo))
            }
        }
        Err(_) => Err(CustomError::Default(Status::InternalServerError)),
    }
}
