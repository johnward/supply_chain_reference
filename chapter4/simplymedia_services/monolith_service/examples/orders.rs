use futures::executor::block_on;
use hyper::{Body, Client, Method, Request, Uri};
use serde_json::{json, to_string_pretty};
use std::env;
use std::io::Read;

pub struct Stock {
    pub id: u64,
    pub product_name: String,
    pub product_id: u64,
    pub amount: u64,
}

fn main() {
    let arg = env::args().nth(1).expect("No args");

    if arg == "create" {
        create_order();
    } else if arg == "list_orders" {
        list_orders();
    } else if arg == "stock" {
        create_stock(Stock {
            id: 1,
            product_name: String::from("Harry Potter"),
            product_id: 22,
            amount: 3,
        });
        create_stock(Stock {
            id: 1,
            product_name: String::from("Hitch Hikers Guide to the Galaxy"),
            product_id: 32,
            amount: 70,
        });
        create_stock(Stock {
            id: 1,
            product_name: String::from("Harry Potter 2"),
            product_id: 40,
            amount: 66,
        });
    }
}

#[tokio::main]
async fn create_order() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:8081/order/create")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"id":0,
            "product_name":"Harry Potter",
            "product_id":3,
            "customer_id": 5,
            "amount":3,
            "address":"4 Privot Drive, London",
            "fulfilled":false
        }"#,
        ))?;

    let client = Client::new();

    let resp = client.request(req).await?;

    println!("Response: {}", resp.status());

    Ok(())
}

#[tokio::main]
async fn list_orders() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Still inside `async fn main`...
    //let client = Client::new();

    // // Parse an `http::Uri`...
    // let uri = "http://localhost:8081/order/list/5".parse()?;

    // // Await the response...
    // let mut resp = client.get(uri).await?;

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:8081/order/list/5")
        .header("content-type", "application/json")
        .body(Body::from(r#""#))?;

    let client = Client::new();

    let resp = client.request(req).await?;

    println!("Response: {:?}", resp.status());

    Ok(())
}

/*
pub struct Stock {
    pub id: u64,
    pub product_name: String,
    pub product_id: u64,
    pub amount: u64,
} */

#[tokio::main]
async fn create_stock(stock: Stock) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let obj = json!({"id":stock.id,
         "product_name":stock.product_name,
         "product_id":stock.product_id,
         "amount":stock.amount,
    });

    let post_body = format!("{}", serde_json::to_string_pretty(&obj).unwrap());

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:8081/stock/create")
        .header("content-type", "application/json")
        .body(Body::from(post_body))?;

    let client = Client::new();

    let resp = client.request(req).await?;

    println!("Response: {}", resp.status());

    Ok(())
}
