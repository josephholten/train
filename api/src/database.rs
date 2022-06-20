use mongodb::{bson::doc, options::ClientOptions, Client};
use crate::models::{User, GameState};

pub struct APIDatabase {
    users: mongodb::Collection<User>,
    games: mongodb::Collection<GameState>
}

impl APIDatabase {
    pub async fn connect() -> Self {
        // connect client to database
        let mut client_options = ClientOptions::parse("mongodb://database")
            .await
            .expect("failed to parse monodb connection string!");
        client_options.app_name = Some("Train API".to_string());
        let client = Client::with_options(client_options)
            .expect("failed to connect to db!");
    
        // ping database
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .expect("failed to connect to database!");
    
        let db = client.database("train");
        APIDatabase { users: db.collection::<User>("users"), games: db.collection("games")}
    }

    pub async fn insert_user(&self, user: &User) 
        -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> 
    {
        self.users.insert_one(user, None).await
    }

    pub async fn find_user_by_username(&self, username: &str) 
        -> Result<Option<User>, mongodb::error::Error> 
    {
        self.users.find_one(doc! {"username": username}, None).await
    }


}