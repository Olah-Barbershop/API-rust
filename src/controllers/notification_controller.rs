use crate::{
    controllers::{MongoRepo, DB_CON_ERR},
    custom::error::CustomError,
    models::notification_model::Notification,
};
use mongodb::{bson::doc, sync::Collection};
use rocket::{http::Status, serde::json::Json, State};

impl MongoRepo {
    pub fn get_notifications(&self) -> Result<Vec<Notification>, String> {
        let col: Collection<Notification> = self.db.collection("notifications");
        let cursors = match col.find(None, None).ok() {
            Some(c) => c,
            None => return Err(DB_CON_ERR.to_string()),
        };
        let notifications = cursors.map(|doc| doc.unwrap()).collect();
        Ok(notifications)
    }
}

#[get("/notifications")]
pub fn get_notifications(db: &State<MongoRepo>) -> Result<Json<Vec<Notification>>, CustomError> {
    let notifications = db.get_notifications();
    match notifications {
        Ok(notifications) => {
            if notifications.is_empty() {
                Err(CustomError::NotFound(
                    Status::NotFound,
                    "notifications".to_string(),
                ))
            } else {
                Ok(Json(notifications))
            }
        }
        Err(error) => Err(CustomError::ServiceUnavailable(
            Status::ServiceUnavailable,
            error,
        )),
    }
}
