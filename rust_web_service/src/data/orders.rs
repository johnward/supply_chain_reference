use crate::data::get_connection;
use crate::models::{NewOrder, Order};
use crate::schema;
use crate::schema::orders::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_order<'a>(conn: &PgConnection, order: &'a Order) -> Result<Order, Error> {
    use schema::orders;

    let new_order = NewOrder {
        product_name: order.product_name.clone(),
        product_id: order.product_id,
        customer_id: order.customer_id,
        amount: order.amount,
        address: order.address.clone(),
    };

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
}

pub fn fulfill_order<'a>(con: &PgConnection, order_id: i32) -> Option<Order> {
    let order = diesel::update(orders.find(order_id))
        .set(fulfilled.eq(true))
        .get_result::<Order>(con)
        .expect(&format!("Unable to find post {}", order_id)); //.get_result();

    println!("Published post {}", order.id);

    Some(order)
}

pub fn update_order<'a>(con: &PgConnection, order: &'a Order) -> Result<Order, Error> {
    let order = diesel::update(orders).set(order).get_result::<Order>(con);

    // For debug
    println!("Published post {:?}", order.as_ref());

    order
}

pub fn delete_order<'a>(con: &PgConnection, order: &'a Order) -> Result<usize, Error> {
    let num_deleted = diesel::delete(orders.find(order.id)).execute(con);

    // For debug
    println!("Deleted {:?} posts", num_deleted.as_ref());

    num_deleted
}

pub fn show_orders(customer_id_needed: i32) -> Result<Vec<Order>, Error> {
    let connection = get_connection();
    let results = orders
        .filter(customer_id.eq(customer_id_needed))
        .limit(5)
        .load::<Order>(&connection);

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.product_name);
    //     println!("----------\n");
    //     println!("{}", post.address);
    // }

    results
}
