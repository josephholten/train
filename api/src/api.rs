use rocket::{State, serde::json::Json, http::Status};
use crate::database::{APIDatabase};
use crate::models::User;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/protected")]
pub fn protected() -> &'static str {
    "Logged in. Accessing protected material (not yet)!\n"
}

#[post("/register", data = "<user>")]
pub async fn register(user: Json<User>, db: &State<APIDatabase>) -> (Status, String) {
    match db.find_user_by_username(&user.username).await {
        Ok(Some(_user)) => (Status::Conflict, "User already exists".to_string()),
        Ok(None)       => { 
            match db.insert_user(&user).await {
                Ok(_)      => (Status::Created, "".to_string()),
                Err(error) => (Status::ServiceUnavailable, format!("Mongodb error: {}", error))
            }},
        Err(error)     => (Status::ServiceUnavailable, format!("Mongodb error: {}", error))
     }
} 