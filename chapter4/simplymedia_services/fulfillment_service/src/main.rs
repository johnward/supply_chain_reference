#[macro_use]
extern crate dotenv;

mod api;
mod services;
mod web_service;

use crate::web_service::WebService;

//use actix_web::{web, App, HttpServer};

/// Runs the main web server and creates all the end points
///
/// fulfill a specific Order: /order/fulfill

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut web_service = WebService::new();
    web_service.start_webserver().await
}
