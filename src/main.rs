#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};
use crate::service::user::UserService;
use std::sync::Arc;
use rocket::fs::{NamedFile};
use crate::service::item::ItemService;
use crate::service::list::ListService;

mod settings;
mod db;
mod model;
mod service;
mod controller;

#[get("/")]
fn index() -> String {
    "hello world".to_string()
}

#[get("/swagger-ui/<file..>")]
async fn swagger(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("swagger-ui/").join(file)).await.ok()
}

#[get("/swagger-ui")]
async fn swagger_index() -> Option<NamedFile> {
    NamedFile::open(Path::new("swagger-ui/index.html")).await.ok()
}

#[launch]
async fn rocket() -> _ {
    let settings = match settings::init() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let db_pool = match db::init(&settings.db).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error connecting database: {}", e);
            std::process::exit(1);
        }
    };
    let user_service = UserService::new(Arc::clone(&db_pool));
    let list_service = ListService::new(Arc::clone(&db_pool));
    let item_service = ItemService::new(Arc::clone(&db_pool));
    rocket::build()
        .manage(user_service)
        .manage(list_service)
        .manage(item_service)
        .mount("/", routes![index, swagger, swagger_index])
        .attach(controller::user::stage())
        .attach(controller::list::stage())
        .attach(controller::item::stage())
}
