use crate::model::item::{CreateItemRequest, Item};
use crate::service::item::ItemService;
use crate::service::list::ListService;
use crate::service::user::UserService;
use crate::service::AppError;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::State;

#[get("/<username>/list/<list_id>/items?<limit>&<page>")]
async fn get_items(
    username: String,
    list_id: i64,
    limit: Option<i32>,
    page: Option<i32>,
    user_service: &State<UserService>,
    list_service: &State<ListService>,
    item_service: &State<ItemService>,
) -> Result<Json<Vec<Item>>, AppError> {
    let limit = limit.unwrap_or(10);
    let page = page.unwrap_or(1);
    let user = user_service.get_user_by_username(username.as_str()).await?;
    let list = list_service.get_list(user.id, list_id).await?;
    let items = item_service
        .list_items(user.id, list.id, limit, page)
        .await?;
    Ok(Json(items))
}

#[post("/<username>/list/<list_id>/items", data = "<request>")]
async fn create_item(
    username: String,
    list_id: i64,
    request: Json<CreateItemRequest>,
    user_service: &State<UserService>,
    list_service: &State<ListService>,
    item_service: &State<ItemService>,
) -> Result<Json<Item>, AppError> {
    let user = user_service.get_user_by_username(username.as_str()).await?;
    let list = list_service.get_list(user.id, list_id).await?;
    let id = item_service
        .create_item(user.id, list.id, &request.0)
        .await?;
    Ok(Json(Item {
        id,
        description: request.0.description,
        done: false,
    }))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("item", |rocket| async {
        rocket.mount("/user", routes![get_items, create_item])
    })
}
