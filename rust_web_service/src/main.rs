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

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config = config::get_config();

    HttpServer::new(|| {
        App::new()
            .service(handlers::order_handler::order_list)
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
            .service(
                web::resource("/stock/create")
                    .route(web::post().to(handlers::stock_handler::stock_create)),
            )
            .service(
                web::resource("/stock/delete")
                    .route(web::post().to(handlers::stock_handler::stock_delete)),
            )
            .service(
                web::resource("/stock/update")
                    .route(web::post().to(handlers::stock_handler::stock_update)),
            )
            .service(
                web::resource("/stock/increment")
                    .route(web::post().to(handlers::stock_handler::stock_increment)),
                    
            )
    })
    .bind(server_config.unwrap())?
    .run()
    .await
}
