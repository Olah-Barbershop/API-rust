use crate::{
    controllers::MongoRepo,
    custom::error::{CustomError, DB_CON_ERR},
    models::service_model::Service,
};
use mongodb::{bson::doc, options::FindOptions, sync::Collection};
use rocket::{http::Status, serde::json::Json, State};

impl MongoRepo {
    pub fn get_services(&self) -> Result<Vec<Service>, String> {
        let col: Collection<Service> = self.db.collection("services");
        let cursors = match col
            .find(
                None,
                FindOptions::builder().projection(doc! {"_id":0}).build(),
            )
            .ok()
        {
            Some(c) => c,
            None => return Err(DB_CON_ERR.to_string()),
        };
        let services = cursors.map(|doc| doc.unwrap()).collect();
        Ok(services)
    }
}

#[get("/services")]
pub fn get_services(db: &State<MongoRepo>) -> Result<Json<Vec<Service>>, CustomError> {
    let services = db.get_services();
    match services {
        Ok(services) => {
            if services.is_empty() {
                Err(CustomError::NotFound(
                    Status::NotFound,
                    "services".to_string(),
                ))
            } else {
                Ok(Json(services))
            }
        }
        Err(error) => Err(CustomError::ServiceUnavailable(
            Status::ServiceUnavailable,
            error,
        )),
    }
}
