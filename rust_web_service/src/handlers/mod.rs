pub const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// Handlers for Order, Product and Stock from the actic web server
pub mod handlers;
pub mod order_handler;
pub mod product_handler;
pub mod stock_handler;
