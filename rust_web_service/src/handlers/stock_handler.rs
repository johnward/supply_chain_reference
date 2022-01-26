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

// Order endpoint
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

// Order endpoint
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

// Order endpoint
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
