mod config;
mod datastore;
mod order;
mod products;
mod web_server;

use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use slab::Slab;
use std::env;
use std::fmt;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .service(web_server::handlers::hello)
            .service(web_server::handlers::echo)
            .service(
                web::resource("/order/create")
                    .route(web::post().to(web_server::order_handler::order_create)),
            )
            .service(
                web::resource("/order/cancel")
                    .route(web::post().to(web_server::order_handler::order_cancel)),
            )
            .service(
                web::resource("/order/update")
                    .route(web::post().to(web_server::order_handler::order_update)),
            )
            .service(
                web::resource("/product/create")
                    .route(web::post().to(web_server::product_handler::product_create)),
            )
            .service(
                web::resource("/product/delete")
                    .route(web::post().to(web_server::product_handler::product_delete)),
            )
            .service(
                web::resource("/product/update")
                    .route(web::post().to(web_server::product_handler::product_update)),
            )
            .route(
                "/product/list",
                web::get().to(web_server::handlers::manual_hello),
            )
            .route("/hey", web::get().to(web_server::handlers::manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
