use crate::schema::clientes::dsl::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::clientes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Clients {
    pub id: i32,
    pub nome: String,
    pub limite: i32,
    pub saldo_inicial: i32,
}

#[allow(dead_code)]
impl Clients {
    pub fn find(connection: &mut PgConnection) -> Vec<Clients> {
        let results = clientes
            .select(Clients::as_select())
            .load(connection)
            .expect("Error loading clientes");

        results
    }

    pub fn find_by_id(
        connection: &mut PgConnection,
        client_id: i32,
    ) -> Result<Clients, diesel::result::Error> {
        let result = clientes
            .select(Clients::as_select())
            .find(client_id)
            .first(connection);

        result
    }
}

#[cfg(test)]
mod tests {
    use diesel::{Connection, PgConnection};

    use super::*;

    #[test]
    fn should_find_all_clients() {
        let connection = &mut establish_connection();
        let results = Clients::find(connection);

        let first_client = results.get(0).unwrap();
        assert_eq!(first_client.id, 1);
        assert_eq!(first_client.nome, "o barato sai caro");
        assert_eq!(first_client.saldo_inicial, 0);
    }

    #[test]
    fn should_find_client_by_id() {
        let connection = &mut establish_connection();
        let client = Clients::find_by_id(connection, 1);
        let client = client.unwrap();
        assert_eq!(client.id, 1);
        assert_eq!(client.nome, "o barato sai caro");
        assert_eq!(client.saldo_inicial, 0);
    }

    pub fn establish_connection() -> PgConnection {
        let database_url = String::from("postgres://admin:123@localhost/rinha");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
