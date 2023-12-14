use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct User{
    #[serde(rename= "_id", skip_serializing_if="Option::is_none")] //if id is set to None, mongo will generate it
    id: Option<ObjectId>,
    username: String,
    email: String,
    password: String,
    op_level: i8 
}

impl User {
    pub fn new(id: Option<ObjectId>, username: String, email: String, password: String, op_level: i8) -> Self {
         Self { id, username, email, password, op_level } 
    }
    pub fn get_username(self) -> String{
        self.username
    }
}


#[derive(Serialize, Deserialize)]
pub struct UserSchema{
    pub username: String,
    pub email: String,
    pub password: String
}