use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use chrono::Utc;
use models::extract::{Extract, Saldo};
use models::transaction_dto::TransactionDTO;
use models::transaction_result::TransactionResult;
use mongodb::{options::ClientOptions, Client};
use repository::clients::Clients;
use repository::transaction::Transaction;

mod models;
mod repository;
#[actix_web::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(transaction).service(extract))
        .bind(("127.0.0.1", 9999))?
        .run()
        .await
}

#[post("/clientes/{id}/transacoes")]
async fn transaction(path: web::Path<i32>, payload: web::Json<TransactionDTO>) -> HttpResponse {
    let connection: Client = establish_connection().await.unwrap();
    let id = path.into_inner();

    let client = match Clients::find_by_id(&connection, id).await {
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

    if payload.tipo.eq("d") && client.saldo.unwrap() < payload.valor {
        return HttpResponse::UnprocessableEntity().finish();
    }

    let current_saldo = client.saldo.unwrap() - payload.valor;

    match Transaction::save_transaction(&connection, payload.0, id).await {
        Ok(t) => t,
        Err(_err) => return HttpResponse::UnprocessableEntity().finish(),
    };

    let _update_saldo = Clients::update_saldo(&connection, client.id, current_saldo).await;

    let result = TransactionResult {
        limite: client.limite,
        saldo: current_saldo,
    };

    HttpResponse::Ok().json(result)
}

#[get("/clientes/{id}/extrato")]
async fn extract(path: web::Path<i32>) -> HttpResponse {
    let connection: Client = establish_connection().await.unwrap();
    let id = path.into_inner();

    let client = match Clients::find_by_id(&connection, id).await {
        Ok(c) => c,
        Err(_err) => return HttpResponse::InternalServerError().finish(),
    };

    if client.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let client = client.unwrap();

    let transactions = Transaction::get_last_transactions(&connection, client.id).await;

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
pub async fn establish_connection() -> Result<Client, mongodb::error::Error> {
    let database_url = String::from("mongodb://admin:123@localhost:27017/");
    let client_options = ClientOptions::parse(&database_url).await.unwrap();
    let client = Client::with_options(client_options);
    client
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde_json::json;

    use super::*;

    #[actix_web::test]
    async fn test_transaction_post() {
        let app = test::init_service(App::new().service(transaction)).await;
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
    }

    #[actix_web::test]
    async fn test_client_not_found() {
        let app = test::init_service(App::new().service(transaction)).await;
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
    }

    #[actix_web::test]
    async fn test_client_less_debit() {
        let app = test::init_service(App::new().service(transaction)).await;
        let req = test::TestRequest::post()
            .uri("/clientes/1/transacoes")
            .set_json(json!({
                "valor": 1000,
                "tipo" : "d",
                "descricao" : "descricao"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status() == 422);
    }

    #[actix_web::test]
    async fn test_get_extract() {
        let app = test::init_service(App::new().service(extract)).await;

        let req = test::TestRequest::get()
            .uri("/clientes/1/extrato")
            .to_request();

        let resp: Extract = test::call_and_read_body_json(&app, req).await;

        assert!(resp.saldo.limite == 100000);
        assert!(resp.saldo.total != 0);
    }
}
