use crate::data::*;
use crate::models::Product;
use crate::services::{create_error, ServiceError, ServiceErrorTypes};

/// The endpoint to get a current list of all products
/// # Arguments
///
///            
/// # Return type
/// * Responder trait or Error
///
pub fn show_products() -> Result<Vec<Product>, ServiceError> {
    match products::show_products() {
        Ok(product) => Ok(product),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Finding Products {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to create a new product
/// # Arguments
///
/// * 'product'
///            
/// # Return type
/// * Product
///
pub fn create_product<'a>(product: &'a Product) -> Result<Product, ServiceError> {
    let connection = get_connection();

    // Create product business logic here,

    // Potentially search cache first.

    // Call the product service
    let product = match products::create_product(&connection, &product) {
        Ok(order) => Ok(order),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Creating Products {}",
            error.to_string()
        ))),
    };

    // send the product to the cache

    product
}

/// The endpoint to delete a new product
/// # Arguments
///
/// * 'product'
///            
/// # Return type
///
pub fn delete_product<'a>(product: &'a Product) -> Result<usize, ServiceError> {
    // Get the appropiate connection
    let connection = get_connection();

    // Now call the delete function in our data interface
    match products::delete_product(&connection, &product) {
        Ok(ret_product) => Ok(ret_product),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Products {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to update a new product
/// # Arguments
///
/// * 'product' -
///            
/// # Return type
///
pub fn update_product<'a>(product: &'a Product) -> Result<Product, ServiceError> {
    // Get the data interface connection
    let connection = get_connection();

    // Call update product data interface
    match products::update_product(&connection, &product) {
        Ok(ret_product) => Ok(ret_product),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Updating Products {}",
            error.to_string()
        ))),
    }
}
