#[macro_use] extern crate rocket;
use rocket::State;
use mongodb::{bson::{doc, Document}, options::{ClientOptions, ReturnDocument}, Client};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/protected")]
fn protected() -> &'static str {
    "Logged in. Accessing protected material!\n"
} 

async fn get_db_connection() -> mongodb::error::Result<mongodb::Client> {
    let mut client_options = ClientOptions::parse("mongodb://database").await?;
    client_options.app_name = Some("Train API".to_string());
    let client = Client::with_options(client_options)?;

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    Ok(client)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db_client = get_db_connection()
        .await.expect("Couldn't connect to Database. Aborting...");
    println!("Database connection successfull.");

    let res = db_client.database("train").collection::<Document>("test").find_one(doc! {"test": 1u32}, None).await;
    match res {
        Ok(_) => println!("Found test document"),
        _ => panic!()
    }
    println!("Test");

    let rocket_options = rocket::Config::figment()
        .merge(("address", "0.0.0.0"));
    let _rocket = rocket::custom(rocket_options)
        .mount("/", routes![index, protected])
        .manage(db_client)
        .launch()
        .await?;
    
    Ok(())
}