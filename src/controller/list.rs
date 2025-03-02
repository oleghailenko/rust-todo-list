use crate::model::list::{CreateListRequest, ListListResponse, ListResponse};
use crate::service::list::ListService;
use crate::service::user::UserService;
use crate::service::AppError;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::State;

#[get("/<username>/list?<limit>&<page>")]
async fn get_lists(
    username: &str,
    limit: Option<i32>,
    page: Option<i32>,
    user_service: &State<UserService>,
    list_service: &State<ListService>,
) -> Result<Json<ListListResponse>, AppError> {
    let limit = limit.unwrap_or(10);
    let page = page.unwrap_or(1);
    let user = user_service.get_user_by_username(username).await?;
    let lists = list_service.get_lists(user.id, limit, page).await?;
    Ok(Json(ListListResponse { lists }))
}

#[get("/<username>/list/<list_id>")]
async fn get_list(
    username: &str,
    list_id: i64,
    user_service: &State<UserService>,
    list_service: &State<ListService>,
) -> Result<Json<ListResponse>, AppError> {
    let user = user_service.get_user_by_username(username).await?;
    let list = list_service.get_list(user.id, list_id).await?;
    Ok(Json(list))
}

#[post("/<username>/list", data="<request>")]
async fn create_list(
    username: &str,
    request: Json<CreateListRequest>,
    user_service: &State<UserService>,
    list_service: &State<ListService>,
) -> Result<Json<ListResponse>, AppError> {
    let user = user_service.get_user_by_username(username).await?;
    let id = list_service.create_list(user.id, &request.0).await?;
    Ok(Json(ListResponse {
        id,
        name: request.0.name,
        total_items: 0,
        done_items: 0
    }))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("list", |rocket| async {
        rocket.mount("/user/", routes![get_lists, create_list, get_list])
    })
}
