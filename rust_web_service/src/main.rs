#[macro_use]
extern crate diesel;
extern crate dotenv;

mod config;
mod data;
mod handlers;
pub mod models;
mod order;
mod products;
mod schema;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::handlers::hello)
            .service(handlers::handlers::echo)
            .service(
                web::resource("/order/create")
                    .route(web::post().to(handlers::order_handler::order_create)),
            )
            .service(
                web::resource("/order/cancel")
                    .route(web::post().to(handlers::order_handler::order_cancel)),
            )
            .service(
                web::resource("/order/update")
                    .route(web::post().to(handlers::order_handler::order_update)),
            )
            .service(
                web::resource("/product/create")
                    .route(web::post().to(handlers::product_handler::product_create)),
            )
            .service(
                web::resource("/product/delete")
                    .route(web::post().to(handlers::product_handler::product_delete)),
            )
            .service(
                web::resource("/product/update")
                    .route(web::post().to(handlers::product_handler::product_update)),
            )
            .route(
                "/product/list",
                web::get().to(handlers::handlers::manual_hello),
            )
            .route("/hey", web::get().to(handlers::handlers::manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
