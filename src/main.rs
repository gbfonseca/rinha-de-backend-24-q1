use mongodb::{options::ClientOptions, Client};

mod repository;
#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {
    println!("Hello World");
}

pub async fn establish_connection() -> Result<Client, mongodb::error::Error> {
    let database_url = String::from("postgres://admin:123@localhost/rinha");
    let client_options = ClientOptions::parse(&database_url).await.unwrap();
    let client = Client::with_options(client_options);
    client
}
