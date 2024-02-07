// pub async fn find_by_id(client: &Client, id: &i32) {}
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::clientes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Clients {
    pub id: i32,
    pub nome: String,
    pub limite: i32,
}
