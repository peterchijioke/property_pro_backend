use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, error::Result as MongoErrorResult, Collection};
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

impl UserModel {
    pub async fn create(
        collection: Collection<UserModel>,
        new_user: UserModel,
    ) -> MongoErrorResult<()> {
        collection.insert_one(new_user, None).await.map(|_| ())
    }

    pub async fn find_by_username(
        collection: Collection<UserModel>,
        email: &str,
    ) -> MongoErrorResult<Option<UserModel>> {
        let filter = doc! { "email":&email };
        collection.find_one(filter, None).await
    }
}
