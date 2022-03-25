use crate::data::*;
use crate::models::{Order, OrderLine};
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
    // Call the create order data interface
    //orders::create_order(&connection, &order)
    match orders::create_order(&order) {
        Ok(order) => Ok(order),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Creating Orders {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to to create a new order for a perticular customer
/// # Arguments
///
/// * 'orderline' - this contains the JSON body data for the new order
///            
/// # Return type
/// * Orderline - Order created
///
pub fn create_orderline<'a>(orderline: &'a OrderLine) -> Result<OrderLine, ServiceError> {
    // Call the create order data interface
    //orders::create_order(&connection, &order)
    match orders::create_orderline(&orderline) {
        Ok(created_orderline) => Ok(created_orderline),
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
    // Call the delete order data interface
    match orders::delete_order(&order) {
        Ok(size) => Ok(size),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Deleting Orders {}",
            error.to_string()
        ))),
    }
}

/// The endpoint to create a cancel an order which deletes it from the database
/// # Arguments
///
/// * 'orderline' - Orderline to delete
///            
/// # Return type
///
/// * usize - number of orders deleted
///
pub fn delete_orderline<'a>(orderline: &'a OrderLine) -> Result<usize, ServiceError> {
    // Call the delete order data interface
    match orders::delete_orderline(&orderline) {
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
    // Call the update order data interface
    match orders::update_order(&order) {
        Ok(updated_order) => Ok(updated_order),
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Error Updating Orders {}",
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
pub fn update_orderline<'a>(orderline: &'a OrderLine) -> Result<OrderLine, ServiceError> {
    // Call the update order data interface
    match orders::update_orderline(&orderline) {
        Ok(updated_orderline) => Ok(updated_orderline),
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
pub fn complete_fulfill_order(id: i32) -> Result<Order, ServiceError> {
    // Get Order to fulfill
    let order = orders::get_orders(id);

    match order {
        Ok(current_order) => {
            let stocks = stock::get_stock(id);

            match stocks {
                Ok(stocks) => {
                    // if order amount is <= stock amount
                    if stocks.len() == 1
                    /*&& current_order.amount <= stocks[0].amount */
                    {
                        //      Decrement stock amount by order amount
                        match stock::increment_stock(stocks[0].id, stocks[0].amount) {
                            Ok(_s) => match orders::fulfill_order(current_order.id) {
                                Ok(order) => Ok(order),
                                Err(error) => create_error(ServiceErrorTypes::InfoNotFound(
                                    format!("Error Updating Orders {}", error.to_string()),
                                )),
                            },
                            Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
                                "Not Found {}",
                                error.to_string()
                            ))),
                        }
                    } else {
                        create_error(ServiceErrorTypes::InfoNotFound(String::from("Not Found")))
                    }
                }
                Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
                    "Not Found {}",
                    error.to_string()
                ))),
            }
        }
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Not Found {}",
            error.to_string()
        ))),
    }
}
