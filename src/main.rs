#[macro_use]
extern crate rocket;

use crate::service::user::UserService;
use std::sync::Arc;

mod settings;
mod db;
mod model;
mod service;
mod controller;

#[get("/")]
fn index() -> String {
    "hello world".to_string()
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
    rocket::build()
        .manage(user_service)
        .mount("/", routes![index])
        .attach(controller::user::stage())
}
