use crate::models::Order;
use slab::Slab;
use std::sync::{Arc, Mutex};

pub struct OrderDatastore {
    order_db: Arc<Mutex<Slab<Order>>>,
}

impl OrderDatastore {
    pub fn new() -> OrderDatastore {
        OrderDatastore {
            order_db: Arc::new(Mutex::new(Slab::new())),
        }
    }

    pub fn add(order: Order) {}

    pub fn remove(order_id: i32) {}

    pub fn update(order: Order) {}
}
