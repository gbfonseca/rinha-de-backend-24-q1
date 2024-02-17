use actix_web::web::Data;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clients {
    pub id: i64,
    pub limite: i64,
    pub saldo_inicial: i64,
    pub saldo: Option<i64>,
}

#[allow(dead_code)]
impl Clients {
    pub async fn find(client: Data<Client>) -> Vec<Clients> {
        let db = client.database("rinha");
        let filter = doc! {};
        let find_options = FindOptions::builder().build();

        let collection: Collection<Clients> = db.collection("clientes");

        let cursor = collection
            .find(filter, find_options)
            .await
            .expect("Houve um erro ao buscar clientes");

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }

    pub async fn find_by_id(
        client: Data<Client>,
        client_id: i64,
    ) -> Result<std::option::Option<Clients>, mongodb::error::Error> {
        let db = client.database("rinha");
        let filter = doc! {"id": client_id};

        let collection: Collection<Clients> = db.collection("clientes");

        let result = collection.find_one(filter, None).await;

        result
    }

    pub async fn update_saldo(
        client_database: Data<Client>,
        client_id: i64,
        value: i64,
    ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
        let db = client_database.database("rinha");
        let filter = doc! {"id": client_id};
        let update = doc! { "$inc": {"saldo": value} };

        let collection: Collection<Clients> = db.collection("clientes");

        collection.update_one(filter, update, None).await
    }
}

#[cfg(test)]
mod tests {

    use crate::config::database::connect_database;

    use super::*;

    #[tokio::test]
    async fn should_find_all_clients() {
        let connection = Data::new(connect_database().await.unwrap());
        let results = Clients::find(connection).await;
        let first_client = results.get(0).unwrap();
        assert_eq!(first_client.id, 1);
        assert_eq!(first_client.saldo_inicial, 0);
    }

    #[tokio::test]
    async fn should_find_client_by_id() {
        let client = Data::new(connect_database().await.unwrap());
        let client = Clients::find_by_id(client, 1).await.unwrap();
        let client = client.unwrap();
        assert_eq!(client.id, 1);
        assert_eq!(client.saldo_inicial, 0);
    }

    #[tokio::test]
    async fn should_return_erro_when_client_not_found() {
        let client = Data::new(connect_database().await.unwrap());
        let client = Clients::find_by_id(client, 6).await.unwrap();
        assert!(client.is_none())
    }
}
