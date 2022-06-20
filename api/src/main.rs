mod api;
mod models;
mod database;

use crate::api::{index, protected, register};
use crate::database::APIDatabase;

#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = APIDatabase::connect().await;

    let rocket_options = rocket::Config::figment()
        .merge(("address", "0.0.0.0"));
    let _rocket = rocket::custom(rocket_options)
        .mount("/", routes![index, protected, register])
        .manage(db)
        .launch()
        .await?;
    
    Ok(())
}