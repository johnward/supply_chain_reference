use serde_derive::{Deserialize, Serialize};

//use crate::data::orders::OrderDatastore;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Order {
//     id: i32,
//     product_name: String,
//     product_id: i32,
//     amount: i32,
//     address: String,
//     fulfilled: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    id: u32,
    sku: String,
    name: String,
    price: f32,
    amount: i32,
}

// pub struct OrderEngine {
//     datastore: OrderDatastore,
// }

// pub impl OrderEngine {
//     pub fn new() {
//         OrderEngine
//     }
// }

pub fn _new_order(_product_name: String, _gtin: String, _amount: u32) {}

pub fn _update_order(_product_name: String, _gtin: String, _amount: u32) {}

pub fn _delete_order(_product_name: String, _gtin: String, _amount: u32) {}
