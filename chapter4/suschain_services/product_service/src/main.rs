#[macro_use]
extern crate diesel;
extern crate dotenv;

mod api;
mod data;
mod models;
mod schema;
mod services;
mod web_service;

use crate::web_service::WebService;

/// Runs the main web server and creates all the end points
///
/// Create a product Order: /product/create
/// Delete a Product: /product/delete
/// Update a Product: /product/update
///
///

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut web_service = WebService::new();
    web_service.start_webserver().await
}
