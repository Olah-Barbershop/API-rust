use crate::{
    controllers::{MongoRepo, DB_CON_ERR},
    custom::error::CustomError,
    models::contactinfo_model::Contactinfo,
};
use mongodb::{bson::doc, options::FindOptions, sync::Collection};
use rocket::{http::Status, serde::json::Json, State};

impl MongoRepo {
    pub fn get_contactinfo(&self) -> Result<Vec<Contactinfo>, String> {
        let col: Collection<Contactinfo> = self.db.collection("contactinfo");
        let cursors = match col
            .find(
                None,
                FindOptions::builder()
                    .projection(doc! {"_id":0})
                    .sort(doc! {"_id": 1})
                    .build(),
            )
            .ok()
        {
            Some(c) => c,
            None => return Err(DB_CON_ERR.to_string()),
        };
        let contactinfo = cursors.map(|doc| doc.unwrap()).collect();
        Ok(contactinfo)
    }
}

#[get("/contactinfo")]
pub fn get_contactinfo(db: &State<MongoRepo>) -> Result<Json<Vec<Contactinfo>>, CustomError> {
    let contactinfo = db.get_contactinfo();
    match contactinfo {
        Ok(contactinfo) => {
            if contactinfo.is_empty() {
                Err(CustomError::NotFound(
                    Status::NotFound,
                    "contactinfo".to_string(),
                ))
            } else {
                Ok(Json(contactinfo))
            }
        }
        Err(error) => Err(CustomError::ServiceUnavailable(
            Status::ServiceUnavailable,
            error,
        )),
    }
}
