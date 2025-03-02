use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateItemRequest {
    pub description: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Item {
    pub id: i64,
    pub description: String,
    pub done: bool
}