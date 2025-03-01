#[macro_use]
extern crate rocket;
mod settings;
mod db;

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
    println!("{:#?}", settings);
    let db_pool = match db::init(&settings.db).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error connecting database: {}", e);
            std::process::exit(1);
        }
    };
    let result = sqlx::query("SELECT 1")
        .fetch_one(&db_pool)
        .await;
    println!("{:#?}", result);
    rocket::build().mount("/", routes![index])
}
