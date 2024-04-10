use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub _id: Option<ObjectId>,
    pub notif_type: String,
    pub date: String,
    pub description: String,
}
