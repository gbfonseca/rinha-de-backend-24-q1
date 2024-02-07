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

    pub fn establish_connection() -> PgConnection {
        let database_url = String::from("postgres://admin:123@localhost/rinha");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
