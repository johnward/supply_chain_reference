use crate::models::{NewOrder, Order};
use crate::schema;
use crate::schema::orders::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod order_datastore;

pub trait Item<T> {
    fn insert();

    fn delete();
}

pub fn get_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB path not set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn create_order<'a>(conn: &PgConnection, order: &'a Order) -> Order {
    use schema::orders;

    let new_order = NewOrder {
        product_name: order.product_name.clone(),
        product_id: order.product_id,
        customer_id: order.customer_id,
        amount: order.amount,
        address: order.address.clone(),
    };

    let ret = diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
        .expect("Error saving new post");

    println!("Ret: {:?}", ret);

    ret
}

pub fn update_order<'a>(con: &PgConnection, order: &'a Order) {
    let order = diesel::update(orders)
        .set(order)
        .get_result::<Order>(con)
        .expect(&format!("Unable to find post {}", order.id)); //.get_result();

    println!("Published post {}", order.id);
}

pub fn delete_order<'a>(con: &PgConnection, order: &'a Order) {
    let num_deleted = diesel::delete(orders.find(order.id))
        .execute(con)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

pub fn show_orders(customer_id_needed: i32) -> Vec<Order> {
    let connection = get_connection();
    let results = orders
        .filter(customer_id.eq(customer_id_needed))
        .limit(5)
        .load::<Order>(&connection)
        .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.product_name);
    //     println!("----------\n");
    //     println!("{}", post.address);
    // }

    results
}
