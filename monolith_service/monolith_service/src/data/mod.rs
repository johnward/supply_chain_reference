use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod orders;
pub mod products;
pub mod stock;

use diesel::result::Error;

pub trait CrudAccess<T> {
    type Item;

    fn create(conn: &PgConnection, stock: &Self::Item) -> Result<Self::Item, Error>;

    fn show(con: &PgConnection, customer_id_needed: i32) -> Result<Vec<Self::Item>, Error>;

    fn update(conn: &PgConnection, stock: &Self::Item) -> Result<Self::Item, Error>;

    fn delete(conn: &PgConnection, stock: &Self::Item) -> Result<Self::Item, Error>;
}

pub fn get_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB path not set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
