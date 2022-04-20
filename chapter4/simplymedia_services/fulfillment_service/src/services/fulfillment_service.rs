use crate::models::{Order, Stock};
use crate::services::{create_error, ServiceError, ServiceErrorTypes};
use chrono::{DateTime, Duration, Utc};
use futures::executor::block_on;
use hyper::{Body, Client, Method, Request, Uri};
use std::error;

//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Fulfilement Service
/// The aim of thie module is to provide fulfilment functionality
///
///
/// 1. Work out availability of items in warehouse
/// 2. Work out the distance of address from fulfilment centre
/// 3. Work out when courier can pick up item to deliver it
/// 4. Request items to picked out of the warehouse and packages for delivery
/// 5. Request Carrier to pick up items and deliver it/them to Customer
/// 6. Confirm when the Items have been delivered
///

fn check_stock_is_available(order: Order) {
    println!("Check Stock is available");
}

fn get_fulfilment_time(order: Order) -> Option<DateTime<Utc>> {
    None
}

fn get_courier_time(order: Order) -> Option<DateTime<Utc>> {
    None
}

fn request_items_to_be_picked(order: Order) -> bool {
    false
}

fn request_delievery(order: Order) {}

fn item_picked(order: Order) {}

// fn has_order_been_delivered() -> Result<Order, ServiceError> {
//     Ok(())
// }

fn get_orders(id: i32) -> Result<Vec<Order>, ServiceError> {
    Ok(get_orders_async(id))
}

#[tokio::main]
async fn get_orders_async(id: i32) -> Result<Vec<Order>, ServiceError> {
    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:8082/order/list")
        .header("content-type", "application/json")
        .body(Body::from(r#""#))?;

    let client = Client::new();
    let resp = client.request(req).await?;

    let (_, body) = resp.into_parts();
    let bytes = hyper::body::to_bytes(body).await?;
    let result = String::from_utf8(bytes.into_iter().collect()).expect("");

    let orders: Vec<Order> = serde_json::from_str(&result)?;

    Ok(orders)
}

fn get_stock(id: i32) -> Result<Vec<Stock>, ServiceError> {
    Ok(get_stock_async(id))
}

#[tokio::main]
async fn get_stock_async(id: i32) -> Result<Vec<Stock>, Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:8082/order/list")
        .header("content-type", "application/json")
        .body(Body::from(r#""#))?;

    let client = Client::new();
    let resp = client.request(req).await?;

    let (_, body) = resp.into_parts();
    let bytes = hyper::body::to_bytes(body).await?;
    let result = String::from_utf8(bytes.into_iter().collect()).expect("");

    let stocks: Vec<Stock> = serde_json::from_str(&result)?;

    Ok(stocks)
}

#[tokio::main]
async fn fulfill_order(id: i32) -> Result<Order, ServiceError> {
    Ok(Order {
        id: 32,
        customer_id: 32,
        address: String::from("test"),
        fulfilled: true,
    })
}

fn increment_stock(id: i32, amount: i32) -> Result<Stock, ServiceError> {
    Ok(increment_stock_async(id, amount))
}

#[tokio::main]
async fn increment_stock_async(id: i32, _amount: i32) -> Result<Stock, ServiceError> {
    Ok(Stock {
        id: 1,
        product_name: String::from("Harry Potter 2"),
        product_id: 40,
        amount: 66,
    })
}

/// Complete the fulfilment of an order and updates the stock balance
/// # Arguments
///
/// * 'id' - Id of the Order being fulfilled
///
/// # Examples
/// let order = Order::new(id: 3,
///                 String::new("Aeroplanes Book"),
///                 32,
///                 5,
///                 4,
///                 String::new("4 Book Street, London"));
///            
/// result = complete_fulfill_order(order.id);
///
pub fn complete_fulfill_order(id: i32) -> Result<Order, ServiceError> {
    // Get Order to fulfill
    let order = get_orders(id);

    match order {
        Ok(current_order) => {
            let stocks = get_stock(id);

            match stocks {
                Ok(stocks) => {
                    // if order amount is <= stock amount
                    if stocks.len() == 1
                    /*&& current_order.amount <= stocks[0].amount */
                    {
                        //      Decrement stock amount by order amount
                        match increment_stock(stocks[0].id, stocks[0].amount) {
                            Ok(_s) => match fulfill_order(current_order.id) {
                                Ok(order) => Ok(order),
                                Err(error) => create_error(ServiceErrorTypes::InfoNotFound(
                                    format!("Error Updating Orders {}", error.to_string()),
                                )),
                            },
                            Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
                                "Not Found {}",
                                error.to_string()
                            ))),
                        }
                    } else {
                        create_error(ServiceErrorTypes::InfoNotFound(String::from("Not Found")))
                    }
                }
                Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
                    "Not Found {}",
                    error.to_string()
                ))),
            }
        }
        Err(error) => create_error(ServiceErrorTypes::InfoNotFound(format!(
            "Not Found {}",
            error.to_string()
        ))),
    }
}
