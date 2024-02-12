use chrono::Utc;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::models::transaction_dto::TransactionDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: Option<String>,
    pub client_id: i32,
}

#[allow(dead_code)]
impl Transaction {
    pub fn new(valor: i32, tipo: String, descricao: String, client_id: i32) -> Transaction {
        let transaction = Transaction {
            descricao,
            valor,
            tipo,
            realizada_em: Some(Utc::now().to_string()),
            client_id,
        };
        transaction
    }
    pub async fn save_transaction(
        client: &Client,
        transaction: TransactionDTO,
        client_id: i32,
    ) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
        let db = client.database("rinha");
        let collection: Collection<Transaction> = db.collection("transaction");

        let transaction = Transaction::new(
            transaction.valor.to_owned(),
            transaction.tipo.to_owned(),
            transaction.descricao.to_owned(),
            client_id,
        );

        let result = collection.insert_one(transaction, None).await;

        result
    }
}

#[cfg(test)]
mod tests {

    use mongodb::{options::ClientOptions, Client};

    use super::*;

    #[tokio::test]
    async fn should_save_transaction() {
        let connection = establish_connection().await.unwrap();

        let transaction = TransactionDTO {
            descricao: String::from("Teste  unitario"),
            tipo: String::from("c"),
            valor: 70,
        };

        Transaction::save_transaction(&connection, transaction, 1)
            .await
            .unwrap();
    }

    pub async fn establish_connection() -> Result<Client, mongodb::error::Error> {
        let database_url = String::from("mongodb://admin:123@localhost:27017/");
        let client_options = ClientOptions::parse(&database_url).await.unwrap();
        let client = Client::with_options(client_options);
        client
    }
}
