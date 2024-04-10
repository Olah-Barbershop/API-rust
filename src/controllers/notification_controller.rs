use crate::{
    controllers::MongoRepo, custom::error::CustomError, models::notification_model::Notification,
};
use rocket::{http::Status, serde::json::Json, State};

#[get("/notifications")]
pub fn get_notifications(db: &State<MongoRepo>) -> Result<Json<Vec<Notification>>, CustomError> {
    let notifications = db.get_notifications();
    match notifications {
        Ok(notifications) => {
            if notifications.is_empty() {
                Err(CustomError::Default(Status::NotFound))
            } else {
                Ok(Json(notifications))
            }
        }
        Err(_) => Err(CustomError::Default(Status::InternalServerError)),
    }
}
