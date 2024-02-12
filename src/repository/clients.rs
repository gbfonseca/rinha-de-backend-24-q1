use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clients {
    pub id: i32,
    pub limite: i32,
    pub saldo_inicial: i32,
    pub saldo: Option<i32>,
}

#[allow(dead_code)]
impl Clients {
    pub async fn find(client: &Client) -> Vec<Clients> {
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
        client: &Client,
        client_id: i32,
    ) -> Result<std::option::Option<Clients>, mongodb::error::Error> {
        let db = client.database("rinha");
        let filter = doc! {"id": client_id};

        let collection: Collection<Clients> = db.collection("clientes");

        let result = collection.find_one(filter, None).await;

        result
    }

    pub async fn update_saldo(
        client_database: &Client,
        client_id: i32,
        saldo: i32,
    ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
        let db = client_database.database("rinha");
        let filter = doc! {"id": client_id};
        let update = doc! { "$set": {"saldo": saldo} };

        let collection: Collection<Clients> = db.collection("clientes");

        collection.update_one(filter, update, None).await
    }
}

#[cfg(test)]
mod tests {

    use mongodb::{options::ClientOptions, Client};

    use super::*;

    #[tokio::test]
    async fn should_find_all_clients() {
        let connection = establish_connection().await.unwrap();
        let results = Clients::find(&connection).await;
        let first_client = results.get(0).unwrap();
        assert_eq!(first_client.id, 1);
        assert_eq!(first_client.saldo_inicial, 0);
    }

    #[tokio::test]
    async fn should_find_client_by_id() {
        let client = establish_connection().await.unwrap();
        let client = Clients::find_by_id(&client, 1).await.unwrap();
        let client = client.unwrap();
        assert_eq!(client.id, 1);
        assert_eq!(client.saldo_inicial, 0);
    }

    #[tokio::test]
    async fn should_return_erro_when_client_not_found() {
        let client = establish_connection().await.unwrap();
        let client = Clients::find_by_id(&client, 6).await.unwrap();
        assert!(client.is_none())
    }

    pub async fn establish_connection() -> Result<Client, mongodb::error::Error> {
        let database_url = String::from("mongodb://admin:123@localhost:27017/");
        let client_options = ClientOptions::parse(&database_url).await.unwrap();
        let client = Client::with_options(client_options);
        client
    }
}
