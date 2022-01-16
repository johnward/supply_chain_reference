mod datastore;

struct Order {
    _id: i32,
    product_name: String,
    amount: i32,
}

struct OrderLine {
    order: Order,
}

fn new_order(product_name: String, gtin: String, amount: u32) {}

fn update_order(product_name: String, gtin: String, amount: u32) {}

fn delete_order(product_name: String, gtin: String, amount: u32) {}
