use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use futures_util::TryStreamExt;
use std::{convert::Infallible, net::SocketAddr};

use crate::products::Product;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::str;
use std::sync::{Arc, Mutex};

use crate::products;

use serde::de;
use serde::{Deserialize, Serialize};
use serde_json;

// Order endpoint
pub async fn product_create(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > crate::handlers::MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Product>(&body)?;
    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

// Order endpoint
pub async fn product_delete(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > crate::handlers::MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Product>(&body)?;
    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

// Order endpoint
pub async fn product_update(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > crate::handlers::MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Product>(&body)?;
    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}
