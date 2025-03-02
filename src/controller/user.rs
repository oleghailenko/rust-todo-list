use crate::model::user::{CreateUserRequest, User};
use crate::service::user::UserService;
use crate::service::AppError;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::State;

#[get("/?<limit>&<page>")]
async fn user_list(
    limit: Option<u16>,
    page: Option<u32>,
    user_service: &State<UserService>
) -> Result<Json<Vec<User>>, AppError> {
    let limit = limit.unwrap_or(10);
    let page = page.unwrap_or(1);
    let vec = user_service
        .user_list(limit, page)
        .await?;

    Ok(Json(vec))
}

#[post("/", data = "<user>")]
async fn create_user(
    user: Json<CreateUserRequest>,
    user_service: &State<UserService>,
) -> Result<Json<User>, AppError> {
    let user_id = user_service.create_user(&user.0).await?;
    Ok(Json(User {
        id: user_id,
        username: user.0.username,
    }))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("user", |rocket| async {
        rocket.mount("/user", routes![user_list, create_user])
    })
}
