use mongodb::bson::oid::ObjectId;
use ::serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub username: String,
    pub password: String
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Authorization<'r> {
//     type Error = AuthorizationError;
// 
//     async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         fn is_valid()
//     }
// }

pub type Domino = (u64, u64);

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    deck: Vec<Domino>,
    trains: Vec<Vec<Domino>>,
    hands: Vec<Vec<Domino>>,
}

