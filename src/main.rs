use diesel::{Connection, PgConnection};

mod repository;
pub mod schema;
#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {
    println!("Hello World");
}

pub fn establish_connection() -> PgConnection {
    let database_url = String::from("postgres://admin:123@localhost/rinha");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
