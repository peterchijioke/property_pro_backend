// use std::env;

// use dotenv::dotenv;
// use mongodb::{error::Result, Client};

// pub async fn get_mongo_client() -> Result<Client> {
//     let mongo_path = env::var("DB_URL").expect("JWT_SECRET must be set");
//     let client = Client::with_uri_str(&mongo_path).await?;
//     println!("MongoDB connected successfully.");
//     Ok(client)
// }

pub mod db;
