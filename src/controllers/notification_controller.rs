use crate::{
    controllers::{MongoRepo, DB_CON_ERR},
    custom::error::CustomError,
    models::notification_model::Notification,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Collection,
};
use rocket::{http::Status, serde::json::Json, State};

fn NOTIF_NOT_FOUND(id: String) -> String {
    format!("Notification with id {} not found", id)
}

impl MongoRepo {
    pub fn get_notification(&self, id: &String) -> Result<Notification, String> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(i) => i,
            Err(error) => return Err(error.to_string()),
        };

        let filter = doc! {"_id": obj_id};
        let col: Collection<Notification> = self.db.collection("notifications");
        let notification = match col.find_one(filter, None).ok() {
            Some(n) => n,
            None => return Err(DB_CON_ERR.to_string()),
        };
        match notification {
            Some(n) => Ok(n),
            None => Err(NOTIF_NOT_FOUND(obj_id.to_string())),
        }
    }

    pub fn get_all_notifications(&self) -> Result<Vec<Notification>, String> {
        let col: Collection<Notification> = self.db.collection("notifications");
        let cursors = match col.find(None, None).ok() {
            Some(c) => c,
            None => return Err(DB_CON_ERR.to_string()),
        };
        let notifications = cursors.map(|doc| doc.unwrap()).collect();
        Ok(notifications)
    }
}

#[get("/notification/<path>")]
pub fn get_notification(
    db: &State<MongoRepo>,
    path: String,
) -> Result<Json<Notification>, CustomError> {
    let id = path;
    if id.is_empty() {
        return Err(CustomError::Default(Status::BadRequest));
    };
    let notification = db.get_notification(&id);
    match notification {
        Ok(notification) => Ok(Json(notification)),
        Err(error) => {
            if error == NOTIF_NOT_FOUND(id) {
                Err(CustomError::ServiceUnavailable(Status::BadRequest, error))
            } else {
                Err(CustomError::ServiceUnavailable(
                    Status::ServiceUnavailable,
                    error,
                ))
            }
        }
    }
}

#[get("/notifications")]
pub fn get_all_notifications(
    db: &State<MongoRepo>,
) -> Result<Json<Vec<Notification>>, CustomError> {
    let notifications = db.get_all_notifications();
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
