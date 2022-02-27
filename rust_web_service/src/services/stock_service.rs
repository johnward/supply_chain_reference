use crate::data::*;
use crate::models::{ReturnInfo, Stock};
use crate::services::{create_error, ServiceError, ServiceErrorTypes};

pub fn increment_stock<'a>(stock_id: i32, amount_change: i32) -> Result<Stock, ServiceError> {
    let connection = get_connection();

    match stock::increment_stock(&connection, stock_id, amount_change) {
        Ok(ret_product) => Ok(ret_product),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Products {}",
            error.to_string()
        ))),
    }
}

pub fn get_stock() -> Result<Vec<Stock>, ServiceError> {
    match stock::show_stock() {
        Ok(ret_product) => Ok(ret_product),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Products {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to create a new stock balance
/// # Arguments
///
/// * 'stock'
///            
/// # Return type
/// * pub fn create_stock<'a>(stock: &'a Stock) -> Stock {
///
pub fn create_stock<'a>(stock: &'a Stock) -> Result<Stock, ServiceError> {
    let connection = get_connection();

    match stock::create_stock(&connection, &stock) {
        Ok(ret_product) => Ok(ret_product),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Products {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to delete stock balance
/// # Arguments
///
/// * 'stock'
///            
/// # Return type
/// * pub fn create_stock<'a>(stock: &'a Stock) -> Stock {
///
pub fn delete_stock<'a>(stock: &'a Stock) -> Result<ReturnInfo, ServiceError> {
    // Delete Order
    let connection = get_connection();

    match stock::delete_stock(&connection, &stock) {
        Ok(ret_product) => Ok(ReturnInfo {
            amount: ret_product,
        }),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Products {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to update a stock balance
/// # Arguments
///
/// * 'stock' - this contains the JSON body data for the stock
///            
/// # Return type
/// * Stock
///
pub fn update_stock<'a>(stock: &'a Stock) -> Result<Stock, ServiceError> {
    // Update Order
    let connection = get_connection();

    match stock::update_stock(&connection, &stock) {
        Ok(ret_product) => Ok(ret_product),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Products {}",
            error.to_string()
        ))),
    }
}
