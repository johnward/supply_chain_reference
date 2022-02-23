use crate::data::stock::*;
use crate::data::*;
use crate::models::Order;
use crate::services::{create_error, ServiceError, ServiceErrorTypes};

pub fn show_orders(customer_id_needed: i32) -> Result<Vec<Order>, ServiceError> {
    match orders::show_orders(customer_id_needed) {
        Ok(order) => Ok(order),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Finding Orders {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to to create a new order for a perticular customer
/// # Arguments
///
/// * 'order' - this contains the JSON body data for the new order
///            
/// # Return type
/// * Order - Order created
///
pub fn create_order<'a>(order: &'a Order) -> Result<Order, ServiceError> {
    // Create a database connection
    let connection = get_connection();

    // Call the create order data interface
    //orders::create_order(&connection, &order)
    match orders::create_order(&connection, &order) {
        Ok(order) => Ok(order),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Creating Orders {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to create a cancel an order which deletes it from the database
/// # Arguments
///
/// * 'order' - Order to delete
///            
/// # Return type
///
/// * usize - number of orders deleted
///
pub fn delete_order<'a>(order: &'a Order) -> Result<usize, ServiceError> {
    // Delete Order
    let connection = get_connection();

    // Call the delete order data interface
    match orders::delete_order(&connection, &order) {
        Ok(size) => Ok(size),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Orders {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to update the information on a current order
/// # Arguments
///
/// * 'order' - Order to update
///            
/// # Return type
/// * Order
///
pub fn update_order<'a>(order: &'a Order) -> Result<Order, ServiceError> {
    // Get the data connection
    let connection = get_connection();

    // Call the update order data interface
    match orders::update_order(&connection, &order) {
        Ok(order) => Ok(order),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Updating Orders {}",
            error.to_string()
        ))),
    }
}

/// Complete the fulfilment of an order and updates the stock balance
/// # Arguments
///
/// * 'id' - Id of the Order being fulfilled
///
/// # Examples
/// let order = Order::new(id: 3,
///                 String::new("Aeroplanes Book"),
///                 32,
///                 5,
///                 4,
///                 String::new("4 Book Street, London"));
///            
/// result = complete_fulfill_order(order.id);
///
pub fn complete_fulfill_order(id: i32) -> Result<(), ()> {
    let connection = get_connection();

    // Get Order amount
    let order = orders::fulfill_order(&connection, id);

    let result = match order {
        Some(o) => {
            let stocks = get_stock(&connection, id);

            if stocks.len() == 1 {
                // if order amount is <= stock amount

                if o.amount <= stocks[0].amount {
                    //      Decrement stock amount by order amount
                    increment_stock(&connection, stocks[0].id, stocks[0].amount);

                    //      Set order to fulfilled.
                    orders::fulfill_order(&connection, o.id);
                }
            }
            Ok(())
        }
        None => Err(()),
    };

    result
}
