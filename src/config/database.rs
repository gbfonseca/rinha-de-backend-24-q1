use std::env;

use mongodb::{options::ClientOptions, Client};

pub async fn connect_database() -> Result<Client, mongodb::error::Error> {
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| String::from("mongodb://localhost:27017/"));
    let mut client_options = ClientOptions::parse(&database_url).await.unwrap();
    client_options.max_pool_size = Some(20);
    let client = Client::with_options(client_options);
    client
}
