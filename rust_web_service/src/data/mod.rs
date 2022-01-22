use crate::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::*;
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

use crate::models::{NewOrder, Order};

pub fn create_post<'a>(conn: &PgConnection, order: &'a Order) -> Order {
    use schema::orders;

    let new_order = order.clone();

    let new_order = NewOrder {
        product_name: order.product_name.clone(),
        product_id: order.product_id,
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

pub fn show_posts(customer_id: i32) -> Vec<Order> {
    use crate::schema::orders::dsl::*;

    let connection = get_connection();
    let results = orders
        .filter(customer_id.eq(customer_id))
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
