use actix_web::{web, HttpResponse, Responder};
use std::sync::mpsc;

pub async fn stop(stop_server: web::Data<mpsc::Sender<()>>) -> impl Responder {
    // make request that sends message through the Sender
    stop_server.send(()).unwrap();

    HttpResponse::NoContent().finish()
}
