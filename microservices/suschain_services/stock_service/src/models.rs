use super::schema::stocks;

use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Stock {
    pub id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Insertable)]
#[table_name = "stocks"]
pub struct NewStock {
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ReturnInfo {
    pub amount: usize,
}
