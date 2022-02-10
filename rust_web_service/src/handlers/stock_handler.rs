use crate::data::stock::*;
use crate::data::*;
use crate::models::Stock;
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use futures::StreamExt;
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct StockIncr {
    id: i32,
    incr_amount: i32,
}

pub async fn stock_increment(data: web::Json<StockIncr>) -> Result<HttpResponse, Error> {
    let connection = get_connection();
    let stock = increment_stock(&connection, data.id, data.incr_amount);

    Ok(HttpResponse::Ok().json(stock))
}

#[get("/stock/list/")]
pub async fn stock_list() -> Result<impl Responder, Error> {
    let stocks = show_stock();
    Ok(HttpResponse::Ok().json(stocks))
}

/// The endpoint to create a new stock balance
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new stock
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn stock_create(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > crate::handlers::MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Stock>(&body)?;
    println!("Success");

    let connection = get_connection();
    let created_stock = create_stock(&connection, &obj);

    Ok(HttpResponse::Ok().json(created_stock)) // <- send response
}

/// The endpoint to delete a stock balance
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the stock
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn stock_delete(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > crate::handlers::MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Stock>(&body)?;

    // Delete Order
    let connection = get_connection();
    delete_stock(&connection, &obj);

    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

/// The endpoint to update a stock balance
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the stock
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn stock_update(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > crate::handlers::MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Stock>(&body)?;

    // Update Order
    let connection = get_connection();
    update_stock(&connection, &obj);

    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::stock_handler;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_create_stock() {
        let mut app = test::init_service(
            App::new()
                .service(
                    web::resource("/stock/create")
                        .route(web::post().to(stock_handler::stock_create)),
                )
                .service(
                    web::resource("/stock/delete")
                        .route(web::post().to(stock_handler::stock_delete)),
                )
                .service(
                    web::resource("/stock/update")
                        .route(web::post().to(stock_handler::stock_update)),
                )
                .service(
                    web::resource("/stock/increment")
                        .route(web::post().to(stock_handler::stock_increment)),
                ),
        )
        .await;

        let payload = r#"{"id":0,
        "product_name":"Harry Potter",
        "product_id":3,
        "customer_id": 5,
        "amount":3,
        "address":"4 Privot Drive, London",
        "fulfilled":false
    }"#
        .as_bytes();

        let req = test::TestRequest::post()
            .uri("http://localhost:8080/order/create")
            .header("content-type", "application/json")
            .set_payload(payload)
            .to_request();

        //let resp: AppState = test::read_response_json(&mut app, req).await;
        //let resp = test::read_response(&mut app, req).await;
        let resp = test::call_service(&mut app, req).await;

        println!("Response: {:?}", resp);

        //assert_eq!(1, 1);
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_index_list_stock() {
        let mut app = test::init_service(
            App::new()
                .service(
                    web::resource("/stock/create")
                        .route(web::post().to(stock_handler::stock_create)),
                )
                .service(
                    web::resource("/stock/delete")
                        .route(web::post().to(stock_handler::stock_delete)),
                )
                .service(
                    web::resource("/stock/update")
                        .route(web::post().to(stock_handler::stock_update)),
                )
                .service(
                    web::resource("/stock/increment")
                        .route(web::post().to(stock_handler::stock_increment)),
                ),
        )
        .await;

        let payload = r#""#.as_bytes();

        let req = test::TestRequest::get()
            .uri("http://localhost:8080/order/list/5")
            .header("content-type", "application/json")
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        println!("Response: {:?}", resp);

        assert!(resp.status().is_client_error());
    }
}

/*

#[tokio::main]
async fn list_orders() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Still inside `async fn main`...
    //let client = Client::new();

    // // Parse an `http::Uri`...
    // let uri = "http://localhost:8080/order/list/5".parse()?;

    // // Await the response...
    // let mut resp = client.get(uri).await?;

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:8080/order/list/5")
        .header("content-type", "application/json").body(Body::from(r#""#))?;

    let client = Client::new();

    let resp = client.request(req).await?;

    println!("Response: {:?}", resp.status());

    Ok(())
}*/
