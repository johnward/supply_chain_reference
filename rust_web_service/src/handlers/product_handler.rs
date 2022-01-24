use crate::data::products::*;
use crate::data::*;
use crate::models::Product;
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use futures::StreamExt;
use serde_json;

#[get("/product/list/")]
async fn product_list() -> Result<impl Responder, Error> {
    let products = show_products();
    Ok(HttpResponse::Ok().json(products))
}

// Order endpoint
pub async fn product_create(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Product>(&body)?;
    println!("Success");

    let connection = get_connection();
    let created_order = create_product(&connection, &obj);

    //show_posts(false);

    Ok(HttpResponse::Ok().json(created_order)) // <- send response
}

// Order endpoint
pub async fn product_delete(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Product>(&body)?;

    // Delete Order
    let connection = get_connection();
    delete_product(&connection, &obj);

    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

// Order endpoint
pub async fn product_update(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Product>(&body)?;

    // Update Order
    let connection = get_connection();
    update_product(&connection, &obj);

    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}
