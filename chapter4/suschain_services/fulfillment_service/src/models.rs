use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub address: String,
    pub fulfilled: bool,
}

// impl Order {
//     pub fn new(
//         id: i32,
//         product_name: String,
//         product_id: i32,
//         customer_id: i32,
//         amount: i32,
//         address: String,
//     ) -> Order {
//         Order {
//             id,
//             product_name,
//             product_id,
//             customer_id,
//             amount,
//             address,
//             fulfilled: false,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOrder {
    pub customer_id: i32,
    pub address: String,
    pub fulfilled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderLine {
    pub id: i32,
    pub order_id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOrderLine {
    pub order_id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OrderDetail {
//     id: i32,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub product_type: String,
    pub amount: i32, // TODO Not sure this is needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub product_name: String,
    pub product_type: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    pub id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewStock {
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnInfo {
    pub amount: usize,
}
