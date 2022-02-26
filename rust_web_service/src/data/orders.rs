use crate::models::{NewOrder, Order};
use crate::schema;
use crate::schema::orders::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use schema::orders;

pub fn create_order<'a>(conn: &PgConnection, order: &'a Order) -> Result<Order, Error> {
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

pub fn fulfill_order<'a>(con: &PgConnection, order_id: i32) -> Result<Order, Error> {
    let order = diesel::update(orders.find(order_id))
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

pub fn delete_order<'a>(con: &PgConnection, order: &'a Order) -> Result<usize, Error> {
    let num_deleted = diesel::delete(orders.find(order.id)).execute(con);

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
        .filter(id.eq(order_id_needed))
        .limit(5)
        .load::<Order>(con)
    {
        Ok(o) => Ok(o[0].clone()),
        Err(e) => Err(e),
    }
}
