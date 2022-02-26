use crate::data::get_connection;
use crate::models::{NewStock, Stock};
use crate::schema;
use crate::schema::stocks::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use schema::stocks;

pub fn create_stock<'a>(conn: &PgConnection, stock: &'a Stock) -> Result<Stock, Error> {
    let new_stock = NewStock {
        product_name: stock.product_name.clone(),
        product_id: stock.product_id,
        amount: stock.amount,
    };

    let ret = diesel::insert_into(stocks::table)
        .values(&new_stock)
        .get_result(conn);

    // Debug Message
    println!("Stock Created: {:?}", ret.as_ref());

    ret
}

/// Returns a person with the name given them
///
/// # Arguments
///
/// * `con`      - connection to the database
/// * `stock_id` - id of stock
/// * `amount`   - number to increment stock by
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use doc::Person;
/// let person = Person::new("name");
/// ```
pub fn increment_stock(
    con: &PgConnection,
    stock_id: i32,
    amount_change: i32,
) -> Result<Stock, Error> {
    let stock = diesel::update(stocks.find(stock_id))
        .set(amount.eq(amount + amount_change))
        .get_result::<Stock>(con);

    // Debug Message
    println!("Incremented Stock: {:?}", stock.as_ref());

    stock
}

pub fn update_stock<'a>(con: &PgConnection, stock: &'a Stock) -> Result<Stock, Error> {
    let stock = diesel::update(stocks).set(stock).get_result::<Stock>(con);

    // Debug Message
    println!("Updated Stock {:?}", stock.as_ref());

    stock
}

pub fn delete_stock<'a>(con: &PgConnection, stock: &'a Stock) -> Result<usize, Error> {
    let num_deleted = diesel::delete(stocks.find(stock.id)).execute(con);

    // Debug Message
    println!("Deleted {:?} posts", num_deleted);

    num_deleted
}

pub fn show_stock() -> Result<Vec<Stock>, Error> {
    let connection = get_connection();
    let results = stocks.limit(10).load::<Stock>(&connection);

    // Debug Message
    println!("Show Stock {:?}", results.as_ref());

    results
}

pub fn get_stock(con: &PgConnection, stock_id: i32) -> Result<Vec<Stock>, Error> {
    let results = stocks.filter(id.eq(stock_id)).limit(5).load::<Stock>(con);

    // Debug Message
    println!("Show Stock {:?}", results.as_ref());

    results
}
