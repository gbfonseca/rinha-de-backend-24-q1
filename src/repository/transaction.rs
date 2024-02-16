use actix_web::web::Data;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::models::transaction_dto::TransactionDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub valor: i64,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: Option<String>,
    pub client_id: i64,
}

#[allow(dead_code)]
impl Transaction {
    pub fn new(valor: i64, tipo: String, descricao: String, client_id: i64) -> Transaction {
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
        client: Data<Client>,
        transaction: TransactionDTO,
        client_id: i64,
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

    pub async fn get_last_transactions(client: Data<Client>, client_id: i64) -> Vec<Transaction> {
        let db = client.database("rinha");
        let filter = doc! {"client_id": client_id};
        let find_options = FindOptions::builder()
            .sort(doc! {"realizada_em": -1})
            .limit(10)
            .build();

        let collection: Collection<Transaction> = db.collection("transaction");

        let cursor = collection
            .find(filter, find_options)
            .await
            .expect("Houve um erro ao buscar clientes");

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }
}

#[cfg(test)]
mod tests {

    use crate::config::database::connect_database;

    use super::*;

    #[tokio::test]
    async fn should_save_transaction() {
        let connection = Data::new(connect_database().await.unwrap());

        let transaction = TransactionDTO {
            descricao: String::from("Teste unitario"),
            tipo: String::from("c"),
            valor: 70,
        };

        Transaction::save_transaction(connection.to_owned(), transaction, 1)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn should_get_last_transactions() {
        let connection = Data::new(connect_database().await.unwrap());
        let client_id = 2;
        let transaction = TransactionDTO {
            descricao: String::from("Teste unitario"),
            tipo: String::from("c"),
            valor: 70,
        };

        Transaction::save_transaction(connection.to_owned(), transaction, client_id)
            .await
            .unwrap();

        let transaction = TransactionDTO {
            descricao: String::from("Teste unitario 2"),
            tipo: String::from("c"),
            valor: 700,
        };

        Transaction::save_transaction(connection.to_owned(), transaction, client_id)
            .await
            .unwrap();

        let result = Transaction::get_last_transactions(connection.to_owned(), client_id).await;

        println!("{:?}", result);

        let first = result.first().unwrap();

        assert!(first.client_id == 2);
        assert_eq!(first.descricao, "Teste unitario 2")
    }
}
