use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserNoPassword {
    pub first_name: String,
    pub email: String,
    pub last_name: String,
    pub phone: String,
}
