use crate::service::*;
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use futures::StreamExt;
use handlebars::Handlebars;
use serde::de::Deserialize;
use serde::Serialize;
use serde_derive::Deserialize;
use std::sync::mpsc;

#[derive(Deserialize, Debug)]
pub struct OrderInfo {
    id: i32,
}

pub const MAX_PAYLOAD_SIZE: usize = 262_144; // max payload size is 256k

pub async fn stop(stop_server: web::Data<mpsc::Sender<()>>) -> impl Responder {
    // make request that sends message through the Sender
    stop_server.send(()).unwrap();

    HttpResponse::NoContent().finish()
}

pub async fn get_payload_bytes(mut payload: web::Payload) -> Result<web::BytesMut, Error> {
    let mut body = web::BytesMut::new();
    while let Some(data_chunk) = payload.next().await {
        // Get the data chunks
        let data_chunk = data_chunk?;

        // limit max size of in-memory payload
        if (body.len() + data_chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&data_chunk);
    }

    Ok(body)
}

/// Generic end point for create, update and delete
/// # Arguments
///
/// * 'payload' - the payload received from the HTTP request
/// * 'func' - the function to call
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn object_crud<T, U>(
    payload: web::Payload,
    func: &dyn Fn(&T) -> U,
) -> Result<HttpResponse, Error>
where
    T: for<'de> Deserialize<'de>,
    U: Serialize,
{
    // payload as bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(bytes) => {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<T>(&bytes)?;

            // Call the stock service function to create stock
            let obj_to_return = func(&obj);

            // Now send back the response
            Ok(HttpResponse::Ok().json(obj_to_return))
        }
        Err(e) => Err(e),
    }
}

/// The endpoint to Complete the fulfilment of an order and updates the stock balance
/// # Arguments
///
/// * 'data' - THis is the JSON strong
///            
/// # Return type
/// * HTTPResponse or Error
///
// pub async fn fulfill_order(data: web::Json<OrderInfo>) -> Result<HttpResponse, Error> {
//     let fulfilled = complete_fulfill_order(data.id);

//     match fulfilled {
//         Ok(_) => Ok(HttpResponse::Ok().finish()),
//         Err(_) => {
//             // return arbitory error for now
//             let return_error = Error::from(());
//             Err(return_error)
//         }
//     }
// }

/// The endpoint to complete to get the current list of orders
/// # Arguments
///
/// * 'customer_id' - The id of the customer to get the list of oorders for
///            
/// # Return type
/// * HTTPResponse or Error
///
#[get("/order/list/{customer_id}")]
pub async fn order_list(customer_id: web::Path<i32>) -> Result<impl Responder, Error> {
    println!("Received /order/list/ {}", customer_id);
    let orders = show_orders(customer_id.into_inner());
    Ok(HttpResponse::Ok().json(orders))
}

/// The endpoint to complete to get the current list of all orders
///            
/// # Return type
/// * HTTPResponse or Error
///
#[get("/order/list")]
pub async fn order_list_all() -> Result<impl Responder, Error> {
    println!("Received /order/list connection");
    let orders = show_all_orders();
    Ok(HttpResponse::Ok().json(orders))
}

/// The endpoint to complete to get the current list of orders
/// # Arguments
///
/// * 'customer_id' - The id of the customer to get the list of oorders for
///            
/// # Return type
/// * HTTPResponse or Error
///
#[get("/orderline/list/{customer_id}")]
pub async fn orderline_list(customer_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let orders = show_orderlines(customer_id.into_inner());
    Ok(HttpResponse::Ok().json(orders))
}

/// The endpoint to complete to get the current list of orders
/// # Arguments
///
/// * 'customer_id' - The id of the customer to get the list of oorders for
///            
/// # Return type
/// * HTTPResponse or Error
///
#[get("/order/display/{customer_id}")]
pub async fn order_display(
    customer_id: web::Path<i32>,
    hb: web::Data<Handlebars<'_>>,
) -> Result<impl Responder, Error> {
    let orders = show_orders(customer_id.into_inner());

    let order_string = serde_json::to_string(&orders).unwrap();

    let body = hb.render("index", &order_string).unwrap();

    Ok(HttpResponse::Ok().body(body))
}

/// The endpoint to to create a new order for a perticular customer
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///

pub async fn order_create(payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("Received order_create connection");
    // payload as bytes
    object_crud(payload, &create_order).await
}

/// The endpoint to to create a new order line for a perticular customer
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///

pub async fn orderline_create(payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("Received orderline_create connection");
    // payload as bytes
    object_crud(payload, &create_orderline).await
}

/// The endpoint to update the information on a current order
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn order_update(payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("Received order_update connection");
    object_crud(payload, &update_order).await
}

/// The endpoint to update the information on a current order
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn orderline_update(payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("Received orderline_update connection");
    object_crud(payload, &update_orderline).await
}

/// The endpoint to create a cancel an order which deletes it from the database
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
///
/// * HTTPResponse or Error
///
pub async fn order_cancel(payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("Received order_cancel connection");
    object_crud(payload, &delete_order).await
}

/// The endpoint to create a cancel an order line which deletes it from the database
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
///
/// * HTTPResponse or Error
///
pub async fn orderline_cancel(payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("Received orderline_cancel connection");
    object_crud(payload, &delete_orderline).await
}
