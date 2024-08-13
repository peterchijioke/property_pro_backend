use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNoPassword {
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub email: String,
    pub last_name: String,
    pub phone: String,
}

impl UserModel {
    pub fn without_password(self) -> UserNoPassword {
        UserNoPassword {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
            email: self.email,
        }
    }
}
