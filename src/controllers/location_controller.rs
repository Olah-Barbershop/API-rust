use crate::{
    controllers::MongoRepo,
    custom::error::{CustomError, DB_CON_ERR},
    models::location_model::Location,
};
use mongodb::{bson::doc, options::FindOptions, sync::Collection};
use rocket::{http::Status, serde::json::Json, State};

impl MongoRepo {
    pub fn get_locations(&self) -> Result<Vec<Location>, String> {
        let col: Collection<Location> = self.db.collection("locations");
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
        let locations = cursors.map(|doc| doc.unwrap()).collect();
        Ok(locations)
    }
}

#[get("/locations")]
pub fn get_locations(db: &State<MongoRepo>) -> Result<Json<Vec<Location>>, CustomError> {
    let locations = db.get_locations();
    match locations {
        Ok(locations) => {
            if locations.is_empty() {
                Err(CustomError::NotFound(
                    Status::NotFound,
                    "locations".to_string(),
                ))
            } else {
                Ok(Json(locations))
            }
        }
        Err(error) => Err(CustomError::ServiceUnavailable(
            Status::ServiceUnavailable,
            error,
        )),
    }
}
