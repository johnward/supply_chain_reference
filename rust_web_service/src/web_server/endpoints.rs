use crate::{Context, Response};
use async_trait::async_trait;
use futures::future::Future;
use hyper::{Method, StatusCode};
use route_recognizer::{Match, Params, Router};
use std::collections::HashMap;

use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

// pub struct EndpointsHandler {
//     endpoints: HashMap<Method, Router>,
// }

// pub impl EndpointsHandler {
//     pub fn new() -> EndpointsHandler {
//         EndpointsHandler {
//             endpoints: HashMap::new(),
//         }
//     }

//     pub fn get()
// }

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
pub fn router() -> Router<Body, Infallible> {
    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(State(100))
        .middleware(Middleware::pre(logger))
        .get("/", home_handler)
        .get("/users/:userId", user_handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}


