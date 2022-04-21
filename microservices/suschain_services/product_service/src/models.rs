use super::schema::products;

use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub product_type: String,
    pub amount: i32, // TODO Not sure this is needed
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub product_name: String,
    pub product_type: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ReturnInfo {
    pub amount: usize,
}
