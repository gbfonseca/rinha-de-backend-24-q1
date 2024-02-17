use actix_web::web::Data;

use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Result};

use actix_web_validator::{Json, JsonConfig};
use chrono::Utc;
use config::database::connect_database;
use models::extract::{Extract, Saldo};
use models::transaction_dto::TransactionDTO;
use models::transaction_result::TransactionResult;
use mongodb::Client;
use repository::clients::Clients;
use repository::transaction::Transaction;
mod config;
mod models;
mod repository;

#[actix_web::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), std::io::Error> {
    let connection: Client = connect_database().await.unwrap();
    let connection_data = Data::new(connection);

    let json_config = JsonConfig::default().error_handler(|err, _req| {
        error::InternalError::from_response(err, HttpResponse::UnprocessableEntity().finish())
            .into()
    });

    println!("Rodando em http://0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(json_config.to_owned())
            .app_data(connection_data.clone())
            .service(transaction)
            .service(extract)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[post("/clientes/{id}/transacoes")]
async fn transaction(
    path: web::Path<i64>,
    payload: Json<TransactionDTO>,
    connection: Data<Client>,
) -> HttpResponse {
    let id = path.into_inner();

    let client = match Clients::find_by_id(connection.to_owned(), id).await {
        Ok(c) => c,
        Err(_err) => return HttpResponse::InternalServerError().finish(),
    };

    if client.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let mut client = client.unwrap();

    if client.saldo.is_none() {
        client.saldo = Some(0)
    }

    let transaction_value = if payload.tipo.eq("d") {
        -payload.valor
    } else {
        payload.valor
    };

    let current_saldo = transaction_value + client.saldo.unwrap();

    if current_saldo < -client.limite || transaction_value % 1 != 0 {
        return HttpResponse::UnprocessableEntity().finish();
    }

    match Transaction::save_transaction(connection.to_owned(), payload.0, id).await {
        Ok(t) => t,
        Err(_err) => return HttpResponse::UnprocessableEntity().finish(),
    };

    let _update_saldo = Clients::update_saldo(connection, client.id, transaction_value).await;

    let result = TransactionResult {
        limite: client.limite,
        saldo: current_saldo,
    };

    HttpResponse::Ok().json(result)
}

#[get("/clientes/{id}/extrato")]
async fn extract(path: web::Path<i64>, connection: Data<Client>) -> HttpResponse {
    let id = path.into_inner();

    let client = match Clients::find_by_id(connection.to_owned(), id).await {
        Ok(c) => c,
        Err(_err) => return HttpResponse::InternalServerError().finish(),
    };

    if client.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let client = client.unwrap();

    let transactions = Transaction::get_last_transactions(connection.to_owned(), client.id).await;

    let extract = Extract {
        saldo: Saldo {
            limite: client.limite,
            total: client.saldo.unwrap_or_else(|| 0),
            data_extrato: Utc::now().to_string(),
        },
        ultimas_transacoes: transactions,
    };

    HttpResponse::Ok().json(extract)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{error, test};
    use serde_json::json;
    mod common;
    use actix_web_validator::JsonConfig;

    #[actix_web::test]
    async fn test_transaction_post() {
        let connection: Client = connect_database().await.unwrap();
        let db_data = Data::new(connection);
        let app = test::init_service(App::new().app_data(db_data).service(transaction)).await;
        let req = test::TestRequest::post()
            .uri("/clientes/1/transacoes")
            .set_json(json!({
                "valor": 1000,
                "tipo" : "c",
                "descricao" : "descricao"
            }))
            .to_request();
        let resp: TransactionResult = test::call_and_read_body_json(&app, req).await;

        assert!(resp.limite == 100000);
        assert!(resp.saldo != 0);
        let _unused = common::after().await;
    }

    #[actix_web::test]
    async fn test_block_floating_value() {
        let connection: Client = connect_database().await.unwrap();
        let db_data = Data::new(connection);
        let json_config = JsonConfig::default().error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::UnprocessableEntity().finish())
                .into()
        });
        let app = test::init_service(
            App::new()
                .app_data(json_config)
                .app_data(db_data)
                .service(transaction),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/clientes/1/transacoes")
            .set_json(json!({
                "valor": 1.2,
                "tipo" : "c",
                "descricao" : "descricao"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status() == 422);
        let _unused = common::after().await;
    }

    #[actix_web::test]
    async fn test_block_tipo() {
        let connection: Client = connect_database().await.unwrap();
        let db_data = Data::new(connection);
        let json_config = JsonConfig::default().error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::UnprocessableEntity().finish())
                .into()
        });
        let app = test::init_service(
            App::new()
                .app_data(json_config)
                .app_data(db_data)
                .service(transaction),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/clientes/1/transacoes")
            .set_json(json!({
                "valor": 1,
                "tipo" : "x",
                "descricao" : "descricao"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status() == 422);
        let _unused = common::after().await;
    }

    #[actix_web::test]
    async fn test_client_not_found() {
        let connection: Client = connect_database().await.unwrap();
        let db_data = Data::new(connection);
        let app = test::init_service(App::new().app_data(db_data).service(transaction)).await;
        let req = test::TestRequest::post()
            .uri("/clientes/6/transacoes")
            .set_json(json!({
                "valor": 1000,
                "tipo" : "c",
                "descricao" : "descricao"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status() == 404);
        let _unused = common::after().await;
    }

    #[actix_web::test]
    async fn test_client_less_debit() {
        let connection: Client = connect_database().await.unwrap();
        let db_data = Data::new(connection);
        let app = test::init_service(App::new().app_data(db_data).service(transaction)).await;
        let req = test::TestRequest::post()
            .uri("/clientes/1/transacoes")
            .set_json(json!({
                "valor": 1000001,
                "tipo" : "d",
                "descricao" : "descricao"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status() == 422);
        let _unused = common::after().await;
    }

    #[actix_web::test]
    async fn test_get_extract() {
        let connection: Client = connect_database().await.unwrap();
        let db_data = Data::new(connection);
        let app = test::init_service(
            App::new()
                .app_data(db_data)
                .service(extract)
                .service(transaction),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/clientes/1/transacoes")
            .set_json(json!({
                "valor": 1000,
                "tipo" : "c",
                "descricao" : "descricao"
            }))
            .to_request();
        let _resp = test::call_service(&app, req).await;

        let req = test::TestRequest::get()
            .uri("/clientes/1/extrato")
            .to_request();

        let resp: Extract = test::call_and_read_body_json(&app, req).await;

        assert!(resp.saldo.limite == 100000);
        assert!(resp.saldo.total != 0);
        let _unused = common::after().await;
    }
}
