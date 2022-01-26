use super::schema::orders;
use super::schema::products;
use super::schema::stocks;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub customer_id: i32,
    pub amount: i32,
    pub address: String,
    pub fulfilled: bool,
}

#[derive(Insertable, Queryable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub product_name: String,
    pub product_id: i32,
    pub customer_id: i32,
    pub amount: i32,
    pub address: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OrderDetail {
//     id: i32,
// }

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub product_type: String,
    pub amount: i32,
}

#[derive(Insertable, Queryable)]
#[table_name = "products"]
pub struct NewProduct {
    pub product_name: String,
    pub product_type: String,
    pub amount: i32,
}

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Stock {
    pub id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Insertable, Queryable)]
#[table_name = "stocks"]
pub struct NewStock {
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}
