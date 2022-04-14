use crate::models::Order;
use chrono::{DateTime, Duration, Utc};
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

fn has_order_been_delivered() -> Result<Order> {}
