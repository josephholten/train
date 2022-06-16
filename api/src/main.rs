#[macro_use] extern crate rocket;
// use mongodb::{bson::doc, options::ClientOptions, Client};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));
    // let mut client_options = ClientOptions::parse("mongodb://localhost")

    rocket::custom(figment).mount("/", routes![index])
}