use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateUserRequest {
    pub username: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub username: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserListResponse {
    pub users: Vec<User>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub lists: i32,
    pub items: i32,
    pub done: i32
}