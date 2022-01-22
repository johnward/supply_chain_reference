use crate::data::*;
use crate::models::{Order, OrderDetail};
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use futures::StreamExt;
use serde_json;
use std::collections::HashMap;
use tera::{Context, Tera};

// // store tera template in application state
// pub async fn order_list(
//     tmpl: web::Data<tera::Tera>,
//     query: web::Query<HashMap<String, String>>,
// ) -> Result<HttpResponse, Error> {
//     let s = if let Some(name) = query.get("name") {
//         // submitted form
//         let mut ctx = tera::Context::new();
//         ctx.insert("name", &name.to_owned());
//         ctx.insert("text", &"Welcome!".to_owned());
//         tmpl.render("user.html", &ctx)
//             .map_err(|_| error::ErrorInternalServerError("Template error"))?
//     } else {
//         tmpl.render("index.html", &tera::Context::new())
//             .map_err(|_| error::ErrorInternalServerError("Template error"))?
//     };
//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }

#[get("/order/{id}")]
async fn order_list(customer_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let orders = show_posts(customer_id);
    println!("id: {}", customer_id);
    Ok(web::Json(orders))
}

// Order endpoint
pub async fn order_create(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Order>(&body)?;
    println!("Success");

    let connection = get_connection();
    let created_order = create_post(&connection, &obj);

    //show_posts(false);

    Ok(HttpResponse::Ok().json(created_order)) // <- send response
}

// Order endpoint
pub async fn order_cancel(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<OrderDetail>(&body)?;
    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

// Order endpoint
pub async fn order_update(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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
    let obj = serde_json::from_slice::<Order>(&body)?;
    println!("Success");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}
