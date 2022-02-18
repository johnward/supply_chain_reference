use crate::data::stock::*;
use crate::data::*;
use crate::models::Order;

pub fn show_orders(customer_id_needed: i32) -> Vec<Order> {
    orders::show_orders(customer_id_needed)
}

/// The endpoint to to create a new order for a perticular customer
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///
pub fn create_order<'a>(order: &'a Order) -> Order {
    // Create a database connection
    let connection = get_connection();

    // Call the create order data interface
    orders::create_order(&connection, &order)
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
pub fn delete_order<'a>(order: &'a Order) -> usize {
    // Delete Order
    let connection = get_connection();

    // Call the delete order data interface
    orders::delete_order(&connection, &order)
}

/// The endpoint to update the information on a current order
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///
pub fn update_order<'a>(order: &'a Order) -> Order {
    // Get the data connection
    let connection = get_connection();

    // Call the update order data interface
    orders::update_order(&connection, &order)
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
