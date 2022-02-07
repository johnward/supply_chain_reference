#[macro_use]
extern crate diesel;
extern crate dotenv;

mod config;
mod data;
mod handlers;
mod models;
mod order;
mod products;
mod schema;
mod web_service;

use actix_web::{web, App, HttpServer};

/// Runs the main web server and creates all the end points
///
/// Create a new order: /order/create
/// Cancel an Order: /order/cancel
/// Update and Order: /order/update
/// fulfill a specific Order: /order/fulfill
/// Create a product Order: /product/create
/// Delete a Product: /product/delete
/// Update a Product: /product/update
/// Create Stock: /stock/create
/// Delete Stock: /stock/delete
/// Update Stock: /stock/update
/// Increment the amount of stock: /stock/increment
///
///

async fn main() -> std::io::Result<()> {
    let web_service = WebService2{server:None};

    web_service.start_service().await;
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let server_config = config::get_config();

//     HttpServer::new(|| {
//         App::new()
//             .service(handlers::order_handler::order_list)
//             .service(
//                 web::resource("/order/create")
//                     .route(web::post().to(handlers::order_handler::order_create)),
//             )
//             .service(
//                 web::resource("/order/cancel")
//                     .route(web::post().to(handlers::order_handler::order_cancel)),
//             )
//             .service(
//                 web::resource("/order/update")
//                     .route(web::post().to(handlers::order_handler::order_update)),
//             )
//             .service(
//                 web::resource("/order/fulfill")
//                     .route(web::post().to(handlers::order_handler::fulfill_order)),
//             )
//             .service(
//                 web::resource("/product/create")
//                     .route(web::post().to(handlers::product_handler::product_create)),
//             )
//             .service(
//                 web::resource("/product/delete")
//                     .route(web::post().to(handlers::product_handler::product_delete)),
//             )
//             .service(
//                 web::resource("/product/update")
//                     .route(web::post().to(handlers::product_handler::product_update)),
//             )
//             .service(
//                 web::resource("/stock/create")
//                     .route(web::post().to(handlers::stock_handler::stock_create)),
//             )
//             .service(
//                 web::resource("/stock/delete")
//                     .route(web::post().to(handlers::stock_handler::stock_delete)),
//             )
//             .service(
//                 web::resource("/stock/update")
//                     .route(web::post().to(handlers::stock_handler::stock_update)),
//             )
//             .service(
//                 web::resource("/stock/increment")
//                     .route(web::post().to(handlers::stock_handler::stock_increment)),
//             )
//     })
//     .bind(server_config.unwrap())?
//     .run()
//     .await
// }
