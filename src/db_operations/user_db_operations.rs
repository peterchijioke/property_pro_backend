// db_operations.rs
use crate::models::user::UserModel;
use futures_util::future::ok;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

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
pub async fn find_user_by_id(
    collection: &Collection<UserModel>,
    id: Option<ObjectId>,
) -> Option<UserModel> {
    let filter = doc! { "_id": id };
    collection.find_one(filter, None).await.ok().flatten()
}

pub async fn update_user(
    collection: &Collection<UserModel>,
    user_id: Option<ObjectId>,
    update_request: UserModel,
) -> Option<UserModel> {
    let filter = doc! { "_id": user_id };

    let update = doc! {
        "$set": {
            "first_name": update_request.first_name,
            "last_name": update_request.last_name,
            "phone": update_request.phone,
            "email": update_request.email,
            "password": update_request.password
        }
    };

    match collection.update_one(filter.clone(), update, None).await {
        Ok(_) => collection.find_one(filter, None).await.ok().flatten(),
        Err(_) => None,
    }
}
