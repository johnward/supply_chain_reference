use crate::data::*;
use crate::models::{ReturnInfo, Stock};

pub fn increment_stock<'a>(stock_id: i32, amount_change: i32) -> Stock {
    let connection = get_connection();
    stock::increment_stock(&connection, stock_id, amount_change)
}

pub fn get_stock() -> Vec<Stock> {
    stock::show_stock()
}

/// The endpoint to create a new stock balance
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new stock
///            
/// # Return type
/// * HTTPResponse or Error
///
pub fn create_stock<'a>(stock: &'a Stock) -> Stock {
    let connection = get_connection();
    stock::create_stock(&connection, &stock)
}

pub fn delete_stock<'a>(stock: &'a Stock) -> ReturnInfo {
    // Delete Order
    let connection = get_connection();
    let num_delete = stock::delete_stock(&connection, &stock);

    ReturnInfo { amount: num_delete }
}

/// The endpoint to update a stock balance
/// # Arguments
///
/// * 'stock' - this contains the JSON body data for the stock
///            
/// # Return type
/// * Stock
///
pub fn update_stock<'a>(stock: &'a Stock) -> Stock {
    // Update Order
    let connection = get_connection();
    stock::update_stock(&connection, &stock)
}
