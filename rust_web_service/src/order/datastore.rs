use crate::order::Order;
use postgres::{Client, Error, NoTls};
//use std::collections::HashMap;

pub fn update_product_amount(product_name: String, amount: i32) -> Result<u64, Error> {
    let mut client = Client::connect("postgresql://psql_user:password@localhost/orders", NoTls)?;

    let row = client.execute(
        "UPDATE orders 
                SET amount = $2
                WHERE product_name = $1",
        &[&product_name, &amount],
    )?;

    Ok(row)
}

pub fn create_order(product_name: String, amount: i32) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://psql_user:password@localhost/orders", NoTls)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS orders (
        id              SERIAL PRIMARY KEY,
        product_name     VARCHAR NOT NULL,
        amount          INTEGER NOT NULL
        )",
    )?;

    let rows = client.query(
        "SELECT amount FROM orders WHERE product_name = $1",
        &[&product_name],
    )?;

    if rows.len() == 0 {
        client.execute(
            "INSERT INTO orders (product_name, amount) VALUES ($1, $2)",
            &[&product_name, &amount],
        )?;
    }

    Ok(())
}

pub fn show_table() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://psql_user:password@localhost/orders", NoTls)?;

    for row in client.query(
        "SELECT id, product_name, address, fulfilled, amount FROM orders",
        &[],
    )? {
        let author = Order {
            _id: row.get(0),
            product_name: row.get(1),
            amount: row.get(2),
            address: row.get(3),
            fulfilled: row.get(4),
        };
        println!("Order {} is from {}", author.product_name, author.amount);
    }

    Ok(())
}

// pub fn delete_all_rows() -> Result<(), Error> {
//     let mut client = Client::connect("postgresql://psql_user:password@localhost/orders",
//                                     NoTls)?;

//     client.query("DELETE FROM orders",)?;

//     Ok(())
// }

// pub fn create_order2() -> Result<(), Error> {
//     let mut client = Client::connect("postgresql://psql_user:password@localhost/orders",
//                                     NoTls)?;

//     client.batch_execute("CREATE TABLE IF NOT EXISTS orders (
//                                         id              SERIAL PRIMARY KEY,
//                                         product_name     VARCHAR NOT NULL,
//                                         amount          INTEGER NOT NULL
//                                         )")?;

//     let mut orders = HashMap::new();
//     orders.insert(String::from("Apples"), 23);
//     orders.insert(String::from("Bananas"), 12);
//     orders.insert(String::from("Cola"), 2);

//     for (key, value) in &orders {
//         let order = Order {
//             _id: 0,
//             product_name: key.to_string(),
//             amount: *value
//         };

//         client.execute(
//                 "INSERT INTO orders (product_name, amount) VALUES ($1, $2)",
//                 &[&order.product_name, &order.amount],
//         )?;
//     }

//     for row in client.query("SELECT id, product_name, amount FROM orders", &[])? {
//         let author = Order {
//             _id: row.get(0),
//             product_name: row.get(1),
//             amount: row.get(2),
//         };
//         println!("Order {} is from {}", author.product_name, author.amount);
//     }

//     Ok(())

// }
