use serde_derive::Deserialize;
mod datastore;

#[derive(Deserialize, Debug)]
pub struct Order {
    _id: i32,
    product_name: String,
    amount: i32,
    address: String,
    fulfilled: String,
}

#[derive(Deserialize, Debug)]
pub struct LineItem {
    id: u32,
    sku: String,
    name: String,
    price: f32,
    amount: i32,
}

pub fn new_order(product_name: String, gtin: String, amount: u32) {}

pub fn update_order(product_name: String, gtin: String, amount: u32) {}

pub fn delete_order(product_name: String, gtin: String, amount: u32) {}
