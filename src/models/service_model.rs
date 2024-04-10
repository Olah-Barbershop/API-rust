use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub amount: u16,
}
