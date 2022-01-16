use crate::{Context, Response};
use async_trait::async_trait;
use futures::future::Future;
use hyper::{Method, StatusCode};
use route_recognizer::{Match, Params, Router};
use std::collections::HashMap;

struct end_points {
    endpoints: HashMap<Method, Router>,
}
