#[macro_use]
extern crate rocket;
mod settings;

#[get("/")]
fn index() -> String {
    "hello world".to_string()
}

#[launch]
fn rocket() -> _ {
    let settings = match settings::init() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    println!("{:#?}", settings);
    rocket::build().mount("/", routes![index])
}
