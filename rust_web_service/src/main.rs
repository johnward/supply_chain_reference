use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

mod config;
mod order;
mod web_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = config::get_config();

    //get the address with the config from the config file, or default local host
    let addr = config
        .map(|config| config.address)
        .or_else(|| Some(([127, 0, 0, 1], 8080).into()))
        .unwrap();

    //let router = web_server::handlers::router();

    // Create a Service from the router above to handle incoming requests.
    //let service = RouterService::new(router).unwrap();

    let service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(web_server::handlers::service_handler))
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
