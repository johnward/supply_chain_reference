use crate::api::core_handler::object_crud;
use crate::services::product_service::*;
use actix_web::{error, web, Error, HttpResponse, Responder};

/// The endpoint to get a current list of all products
/// # Arguments
///
///            
/// # Return type
/// * Responder trait or Error
///
pub async fn product_list() -> Result<impl Responder, Error> {
    match show_products() {
        Ok(stocks) => Ok(HttpResponse::Ok().json(stocks)),
        Err(error) => Err(error::ErrorBadRequest(error)),
    }
}

/// The endpoint to create a new product
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new product
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn product_create(payload: web::Payload) -> Result<HttpResponse, Error> {
    object_crud(payload, &create_product).await
}

/// The endpoint to delete a new product
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new product to delete
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn product_delete(payload: web::Payload) -> Result<HttpResponse, Error> {
    object_crud(payload, &delete_product).await
}

/// The endpoint to update a new product
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new product to update
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn product_update(payload: web::Payload) -> Result<HttpResponse, Error> {
    object_crud(payload, &update_product).await
}
