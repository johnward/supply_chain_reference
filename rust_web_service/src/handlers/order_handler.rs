use crate::data::orders::*;
use crate::data::*;
use crate::models::Order;
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use futures::StreamExt;
use serde_json;

#[get("/order/list/{customer_id}")]
async fn order_list(customer_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let orders = show_orders(customer_id.into_inner());
    Ok(HttpResponse::Ok().json(orders))
}

// Order endpoint
pub async fn order_create(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Order>(&body)?;
    println!("Success");

    let connection = get_connection();
    let created_order = create_order(&connection, &obj);

    //show_posts(false);

    Ok(HttpResponse::Ok().json(created_order)) // <- send response
}

// Order endpoint
pub async fn order_cancel(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Order>(&body)?;

    // Delete Order
    let connection = get_connection();
    delete_order(&connection, &obj);

    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

// Order endpoint
pub async fn order_update(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Order>(&body)?;

    // Update Order
    let connection = get_connection();
    update_order(&connection, &obj);

    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}
