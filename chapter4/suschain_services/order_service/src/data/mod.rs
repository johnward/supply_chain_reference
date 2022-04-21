use crate::models::{NewOrder, NewOrderLine, Order, OrderLine};
use crate::schema;
use crate::schema::orderlines::dsl::*;
use crate::schema::orders::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use schema::orderlines;
use schema::orders;
use std::env;

pub fn get_connection() -> PgConnection {
    dotenv().ok();

    let data_url = env::var("DATABASE_URL").expect("DB path not set");

    PgConnection::establish(&data_url).expect(&format!("Error connecting to {}", data_url))
}

pub fn create_order_data<'a>(order: &'a Order) -> Result<Order, Error> {
    let connection = get_connection();

    let new_order = NewOrder {
        customer_id: order.customer_id,
        address: order.address.clone(),
        fulfilled: false,
    };

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(&connection)
}

pub fn create_orderline_data<'a>(orderline: &'a OrderLine) -> Result<OrderLine, Error> {
    let new_orderline = NewOrderLine {
        order_id: orderline.order_id,
        product_name: orderline.product_name.clone(),
        product_id: orderline.product_id,
        amount: orderline.amount,
    };

    let connection = get_connection();

    diesel::insert_into(orderlines::table)
        .values(&new_orderline)
        .get_result(&connection)
}

// pub fn fulfill_order<'a>(ord_id: i32) -> Result<Order, Error> {
//     let connection = get_connection();

//     let order = diesel::update(orders.find(ord_id))
//         .set(fulfilled.eq(true))
//         .get_result::<Order>(&connection);

//     println!("Published post {:?}", order.as_ref());

//     order
// }

pub fn update_order_data<'a>(order: &'a Order) -> Result<Order, Error> {
    let connection = get_connection();

    let order = diesel::update(orders::table)
        .set(order)
        .get_result::<Order>(&connection);

    // For debug
    println!("Published post {:?}", order.as_ref());

    order
}

pub fn update_orderline_data<'a>(orderline: &'a OrderLine) -> Result<OrderLine, Error> {
    let connection = get_connection();

    let orderline = diesel::update(orderlines::table)
        .set(orderline)
        .get_result::<OrderLine>(&connection);

    // For debug
    println!("Published post {:?}", orderline.as_ref());

    orderline
}

pub fn delete_order_data<'a>(order: &'a Order) -> Result<usize, Error> {
    let connection = get_connection();

    let num_deleted = diesel::delete(orders.find(order.id)).execute(&connection);

    // For debug
    println!("Deleted {:?} posts", num_deleted.as_ref());

    num_deleted
}

pub fn delete_orderline_data<'a>(orderline: &'a OrderLine) -> Result<usize, Error> {
    let connection = get_connection();

    let num_deleted = diesel::delete(orderlines.find(orderline.id)).execute(&connection);

    // For debug
    println!("Deleted {:?} posts", num_deleted.as_ref());

    num_deleted
}

pub fn show_orders_data(customer_id_needed: i32) -> Result<Vec<Order>, Error> {
    let connection = get_connection();

    orders
        .filter(customer_id.eq(customer_id_needed))
        .limit(5)
        .load::<Order>(&connection)
}

pub fn show_orderlines_data(order_id_needed: i32) -> Result<Vec<OrderLine>, Error> {
    let connection = get_connection();

    orderlines
        .filter(order_id.eq(order_id_needed))
        .limit(5)
        .load::<OrderLine>(&connection)
}

pub fn _get_orders_data(order_id_needed: i32) -> Result<Order, Error> {
    let connection = get_connection();

    match orders
        .filter(orders::id.eq(order_id_needed))
        .limit(5)
        .load::<Order>(&connection)
    {
        Ok(o) => Ok(o[0].clone()),
        Err(e) => Err(e),
    }
}

pub fn get_all_orders_data() -> Result<Vec<Order>, Error> {
    let connection = get_connection();

    match orders.load::<Order>(&connection) {
        Ok(o) => {
            println!("Order Retrieved: {:?}", o);
            Ok(o)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::*;
    use crate::models::{Order, OrderLine};

    #[test]
    fn create_and_view_orderlines() {
        let orders = match get_all_orders_data() {
            Ok(orders) => orders,
            Err(_error) => {
                assert!(false);
                let mut vec = Vec::new();
                vec.push(Order {
                    id: 0,
                    customer_id: 1,
                    address: String::from("37 Woodchip Road, Manchester"),
                    fulfilled: false,
                });
                vec
            }
        };

        let order_item = OrderLine {
            id: 0,
            order_id: orders[0].id,
            product_name: String::from("Horry Potter 1"),
            product_id: 32,
            amount: 1,
        };

        let created_orderline = match create_orderline_data(&order_item) {
            Ok(created_orderline) => {
                assert_eq!(created_orderline.product_name, order_item.product_name);
                assert_eq!(created_orderline.product_id, order_item.product_id);
                assert_eq!(created_orderline.amount, order_item.amount);

                created_orderline
            }
            Err(_error) => {
                assert!(false);

                OrderLine {
                    id: 0,
                    order_id: 0,
                    product_name: String::from(""),
                    product_id: 0,
                    amount: 0,
                }
            }
        };

        match delete_orderline_data(&created_orderline) {
            Ok(size) => assert_eq!(size, 1),
            Err(_error) => assert!(false),
        }
    }

    #[test]
    fn test_create_delete_order() {
        let new_order = Order {
            id: 0,
            customer_id: 1,
            address: String::from("37 Woodchip Road, Manchester"),
            fulfilled: false,
        };

        // Call the create order data interface
        //orders::create_order(&connection, &order)
        let create_order = match create_order_data(&new_order) {
            Ok(created_order) => {
                assert_eq!(created_order.customer_id, new_order.customer_id);
                assert_eq!(created_order.address, new_order.address);
                assert_eq!(created_order.fulfilled, new_order.fulfilled);

                created_order
            }
            Err(_error) => {
                assert!(false);

                Order {
                    id: 0,
                    customer_id: 0,
                    address: String::from(""),
                    fulfilled: false,
                }
            }
        };

        match delete_order_data(&create_order) {
            Ok(size) => assert_eq!(size, 1),
            Err(_error) => assert!(false),
        }
    }

    #[test]
    fn test_update_order() {
        let new_order = Order {
            id: 0,
            customer_id: 1,
            address: String::from("37 Woodchip Road, Manchester"),
            fulfilled: false,
        };

        // Call the create order data interface
        let mut created_order = match create_order_data(&new_order) {
            Ok(created_order) => {
                assert_eq!(created_order.customer_id, new_order.customer_id);
                assert_eq!(created_order.address, new_order.address);
                assert_eq!(created_order.fulfilled, new_order.fulfilled);

                created_order
            }
            Err(_error) => {
                assert!(false);

                Order {
                    id: 0,
                    customer_id: 0,
                    address: String::from(""),
                    fulfilled: false,
                }
            }
        };

        created_order.address = String::from("1 Woodchip Road, Manchester");

        match update_order_data(&created_order) {
            Ok(updated_order) => assert_eq!(
                updated_order.address,
                String::from("1 Woodchip Road, Manchester")
            ),
            Err(_error) => assert!(false),
        }

        match delete_order_data(&created_order) {
            Ok(size) => assert_eq!(size, 1),
            Err(_error) => assert!(false),
        }
    }
}
