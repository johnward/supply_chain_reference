use futures::executor::block_on;
use std::env;
use std::io::Read;

use hyper::{Body, Client, Method, Request, Uri};

fn main() {
    let arg = env::args().nth(1).expect("No args");

    if arg == "create" {
        create();
    } else if arg == "list_orders" {
        list_orders();
    }
}

#[tokio::main]
async fn create() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:8080/order/create")
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
    // let uri = "http://localhost:8080/order/list/5".parse()?;

    // // Await the response...
    // let mut resp = client.get(uri).await?;

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:8080/order/list/5")
        .header("content-type", "application/json").body(Body::from(r#""#))?;

    let client = Client::new();

    let resp = client.request(req).await?;

    println!("Response: {:?}", resp.status());

    Ok(())
}
