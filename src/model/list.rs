use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateListRequest {
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct List {
    pub id: i64,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ListResponse {
    pub id: i64,
    pub name: String,
    pub total_items: i32,
    pub done_items: i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ListListResponse {
    pub lists: Vec<ListResponse>,
}