use crate::api::core_handler::object_crud;
use crate::services::fulfillment_service::*;
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
