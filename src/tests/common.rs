use mongodb::{options::ClientOptions, Client};

use crate::repository::clients::Clients;

async fn insert_data(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    insert_document(&client, 0, 1000 * 100, 1).await?;
    insert_document(&client, 0, 800 * 100, 2).await?;
    insert_document(&client, 0, 10000 * 100, 3).await?;
    insert_document(&client, 0, 100000 * 100, 4).await?;
    insert_document(&client, 0, 5000 * 100, 5).await?;

    Ok(())
}

async fn insert_document(
    client: &Client,
    saldo_inicial: i64,
    limite: i64,
    id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Obter a coleção "clientes"
    let collection = client.database("rinha").collection::<Clients>("clientes");

    // Criar um documento  para inserção
    let document = Clients {
        saldo_inicial,
        limite,
        id,
        saldo: Some("0".parse::<i64>().unwrap()),
    };

    // Inserir o documento na coleção
    collection.insert_one(document, None).await?;

    Ok(())
}

async fn clear_collection(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let clients = client.database("rinha").collection::<Clients>("clientes");
    let transactions = client
        .database("rinha")
        .collection::<Clients>("transactions");

    // Limpar (drop) a coleção
    clients.drop(None).await?;
    transactions.drop(None).await?;

    println!("Coleção limpa com sucesso!");

    Ok(())
}

pub async fn after() -> Result<(), Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    let client = Client::with_options(client_options)?;
    let _clear_collection = clear_collection(&client).await;
    insert_data(&client).await
}
