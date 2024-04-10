use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contactinfo {
    pub left: Contact,
    pub right: Contact,
}
