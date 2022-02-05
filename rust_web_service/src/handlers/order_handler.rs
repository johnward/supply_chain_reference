use crate::data::orders::*;
use crate::data::*;
use crate::models::Order;
use crate::order::complete_fulfill_order;
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use futures::StreamExt;
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    id: i32,
}

/// The endpoint to Complete the fulfilment of an order and updates the stock balance
/// # Arguments
///
/// * 'data' - THis is the JSON strong
///            
/// # Return type
/// * HTTPResponse or Error
/// 
pub async fn fulfill_order(data: web::Json<OrderInfo>) -> Result<HttpResponse, Error> {
    let fulfilled = complete_fulfill_order(data.id);

    match fulfilled {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => {
            // return arbitory error for now
            let return_error = Error::from(());
            Err(return_error)
        }
    }
}

/// The endpoint to complete to get the current list of orders
/// # Arguments
///
/// * 'customer_id' - The id of the customer to get the list of oorders for
///            
/// # Return type
/// * HTTPResponse or Error
/// 
#[get("/order/list/{customer_id}")]
async fn order_list(customer_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let orders = show_orders(customer_id.into_inner());
    Ok(HttpResponse::Ok().json(orders))
}

/// The endpoint to to create a new order for a perticular customer
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
/// 
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

/// The endpoint to create a cancel an order which deletes it from the database
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
/// 
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

/// The endpoint to update the information on a current order
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
/// 
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
