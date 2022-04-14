use crate::api::core_handler::object_crud;
use crate::services::order_service::*;
use actix_web::{get, web, Error, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
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
pub async fn order_list(customer_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let orders = show_orders(customer_id.into_inner());
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
    object_crud(payload, &delete_orderline).await
}
