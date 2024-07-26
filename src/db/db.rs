use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Collection, Database};
use std::env;

pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub async fn new() -> Self {
        dotenv().ok();
        let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
        let mut client_options = ClientOptions::parse(&mongo_uri)
            .await
            .expect("Failed to parse options");
        client_options.app_name = Some("propertyPro".to_string());

        let client = Client::with_options(client_options).expect("Failed to initialize client");
        let db = client.database("propertyPro");
        AppState { db }
    }
}
