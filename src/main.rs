mod repository;
pub mod schema;

use diesel::prelude::*;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {
    use self::schema::clientes::dsl::*;

    let connection = &mut establish_connection();

    let results = clientes
        .select(crate::repository::clients::Clients::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for client in results {
        println!("{}", client.nome);
        println!("-----------\n");
        println!("{}", client.limite);
    }

    println!("Hello World");
}

pub fn establish_connection() -> PgConnection {
    let database_url = String::from("postgres://admin:123@localhost/rinha");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
