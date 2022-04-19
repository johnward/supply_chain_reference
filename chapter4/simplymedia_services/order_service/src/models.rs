use super::schema::orderlines;
use super::schema::orders;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Identifiable, Queryable, AsChangeset, Debug, Serialize, Deserialize, PartialEq)]
#[table_name = "orders"]
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

#[derive(Insertable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub customer_id: i32,
    pub address: String,
    pub fulfilled: bool,
}

#[derive(
    Clone, Identifiable, Associations, Queryable, AsChangeset, Debug, Serialize, Deserialize,
)]
#[belongs_to(Order)]
#[table_name = "orderlines"]
pub struct OrderLine {
    pub id: i32,
    pub order_id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Insertable, Associations)]
#[belongs_to(Order)]
#[table_name = "orderlines"]
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

#[derive(Serialize, Deserialize)]
pub struct ReturnInfo {
    pub amount: usize,
}
