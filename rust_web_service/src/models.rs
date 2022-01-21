use super::schema::orders;

#[derive(Queryable)]
pub struct Order {
    pub id: i32,
    pub product_name: String,
    pub product_id: i32,
    pub amount: i32,
    pub address: String,
    pub fulfilled: bool,
}

#[derive(Insertable)]
#[table_name = "orders"]
pub struct OrderDetail {
    pub id: i32,
}
