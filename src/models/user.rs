use mongodb::{
    bson::{doc, Document},
    error::Result,
    Client, Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel {
    pub first_name: String,
    pub email: String,
    pub password: String,
    pub last_name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserNoPassword {
    pub first_name: String,
    pub email: String,
    pub last_name: String,
    pub phone: String,
}

impl UserModel {
    fn collection(client: &Client) -> Collection<UserModel> {
        client.database("mydb").collection("users")
    }

    pub async fn create(client: &Client, new_user: UserModel) -> Result<()> {
        let collection = Self::collection(client);
        collection.insert_one(new_user, None).await.map(|_| ())
    }

    pub async fn find_by_username(client: &Client, username: &str) -> Result<Option<UserModel>> {
        let collection = Self::collection(client);
        let filter = doc! { "username": username };
        collection.find_one(filter, None).await
    }
}
