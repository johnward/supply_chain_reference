use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use futures_util::TryStreamExt;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

mod config;
mod order;
mod web_server;

use serde::de;
use serde::{Deserialize, Serialize};
use serde_json;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Debug, Serialize, Deserialize)]
struct MyObj2 {
    name: String,
    number: i32,
}

/// This handler manually load request payload and parse json object
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj2>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web_server::handlers::hello)
            .service(web_server::handlers::echo)
            .service(
                web::resource("/manual").route(web::post().to(web_server::handlers::index_manual)),
            )
            .route("/hey", web::get().to(web_server::handlers::manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
