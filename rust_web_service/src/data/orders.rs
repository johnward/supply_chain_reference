use crate::models::{NewOrder, NewOrderLine, Order, OrderLine};
use crate::schema;
use crate::schema::orderlines::dsl::*;
use crate::schema::orders::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use schema::orderlines;
use schema::orders;

pub fn create_order<'a>(conn: &PgConnection, order: &'a Order) -> Result<Order, Error> {
    let new_order = NewOrder {
        customer_id: order.customer_id,
        address: order.address.clone(),
        fulfilled: false,
    };

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
}

pub fn create_order_line<'a>(
    conn: &PgConnection,
    orderline: &'a OrderLine,
) -> Result<OrderLine, Error> {
    let new_orderline = NewOrderLine {
        order_id: orderline.order_id,
        product_name: orderline.product_name.clone(),
        product_id: orderline.product_id,
        amount: orderline.amount,
    };

    diesel::insert_into(orderlines::table)
        .values(&new_orderline)
        .get_result(conn)
}

pub fn fulfill_order<'a>(con: &PgConnection, ord_id: i32) -> Result<Order, Error> {
    let order = diesel::update(orders.find(ord_id))
        .set(fulfilled.eq(true))
        .get_result::<Order>(con);

    println!("Published post {:?}", order.as_ref());

    order
}

pub fn update_order<'a>(con: &PgConnection, order: &'a Order) -> Result<Order, Error> {
    let order = diesel::update(orders::table)
        .set(order)
        .get_result::<Order>(con);

    // For debug
    println!("Published post {:?}", order.as_ref());

    order
}

pub fn update_orderline<'a>(
    con: &PgConnection,
    orderline: &'a OrderLine,
) -> Result<OrderLine, Error> {
    let orderline = diesel::update(orderlines::table)
        .set(orderline)
        .get_result::<OrderLine>(con);

    // For debug
    println!("Published post {:?}", orderline.as_ref());

    orderline
}

pub fn delete_order<'a>(con: &PgConnection, order: &'a Order) -> Result<usize, Error> {
    let num_deleted = diesel::delete(orders.find(order.id)).execute(con);

    // For debug
    println!("Deleted {:?} posts", num_deleted.as_ref());

    num_deleted
}

pub fn delete_orderline<'a>(con: &PgConnection, orderline: &'a OrderLine) -> Result<usize, Error> {
    let num_deleted = diesel::delete(orderlines.find(orderline.id)).execute(con);

    // For debug
    println!("Deleted {:?} posts", num_deleted.as_ref());

    num_deleted
}

pub fn show_orders(con: &PgConnection, customer_id_needed: i32) -> Result<Vec<Order>, Error> {
    let results = orders
        .filter(customer_id.eq(customer_id_needed))
        .limit(5)
        .load::<Order>(con);

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.product_name);
    //     println!("----------\n");
    //     println!("{}", post.address);
    // }

    results
}

pub fn get_orders(con: &PgConnection, order_id_needed: i32) -> Result<Order, Error> {
    match orders
        .filter(orders::id.eq(order_id_needed))
        .limit(5)
        .load::<Order>(con)
    {
        Ok(o) => Ok(o[0].clone()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::*;
    use crate::models::Order;

    #[test]
    fn test_create_delete_order() {
        // Create a database connection
        let connection = get_connection();

        let new_order = Order {
            id: 0,
            customer_id: 1,
            address: String::from("37 Woodchip Road, Manchester"),
            fulfilled: false,
        };

        // Call the create order data interface
        //orders::create_order(&connection, &order)
        match orders::create_order(&connection, &new_order) {
            Ok(created_order) => {
                assert_eq!(created_order.customer_id, new_order.customer_id);
                assert_eq!(created_order.address, new_order.address);
                assert_eq!(created_order.fulfilled, new_order.fulfilled);
            }
            Err(_error) => assert!(false),
        }

        match orders::delete_order(&connection, &new_order) {
            Ok(size) => assert_eq!(size, 1),
            Err(_error) => assert!(false),
        }
    }

    #[test]
    fn test_update_order() {
        // Create a database connection
        let connection = get_connection();

        let mut new_order = Order {
            id: 0,
            customer_id: 1,
            address: String::from("37 Woodchip Road, Manchester"),
            fulfilled: false,
        };

        // Call the create order data interface
        match orders::create_order(&connection, &new_order) {
            Ok(created_order) => {
                assert_eq!(created_order.customer_id, new_order.customer_id);
                assert_eq!(created_order.address, new_order.address);
                assert_eq!(created_order.fulfilled, new_order.fulfilled);
            }
            Err(_error) => assert!(false),
        }

        new_order.address = String::from("1 Woodchip Road, Manchester");

        match orders::update_order(&connection, &new_order) {
            Ok(updated_order) => assert_eq!(
                updated_order.address,
                String::from("1 Woodchip Road, Manchester")
            ),
            Err(_error) => assert!(false),
        }

        match orders::delete_order(&connection, &new_order) {
            Ok(size) => assert_eq!(size, 1),
            Err(_error) => assert!(false),
        }
    }
}
