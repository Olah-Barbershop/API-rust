use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    pub phone_number: String,
    pub monday_to_thursday: String,
    pub friday: String,
    pub saturday_to_sunday: String,
}
