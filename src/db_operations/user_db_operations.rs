// db_operations.rs
use crate::models::user::UserModel;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection};

pub async fn insert_user(
    collection: &Collection<UserModel>,
    user: UserModel,
) -> mongodb::error::Result<()> {
    collection.insert_one(user, None).await.map(|_| ())
}

pub async fn find_user_by_email(
    collection: &Collection<UserModel>,
    email: &str,
) -> Option<UserModel> {
    let filter = doc! { "email": email };
    collection.find_one(filter, None).await.ok().flatten()
}
