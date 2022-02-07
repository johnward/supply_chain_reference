use crate::config;
use crate::handlers::{order_handler, product_handler, stock_handler};
use actix_web::{web, App, HttpServer};

// struct CustomServer {
//     srv: HttpServer<App, fn() -> App>,
// }

// impl CustomServer {
//     fn new() -> CustomServer {
//         CustomServer {
//             srv: HttpServer::new(|| App),
//         }
//     }
// }
// Make async: https://stackoverflow.com/questions/58173711/how-can-i-store-an-async-function-in-a-struct-and-call-it-from-a-struct-instance

// pub struct Service {
//     name: String,
// }

// impl Service {
//     pub fn new(name: &String) -> Service {
//         Service { name: name.clone() }
//     }
// }

// async fn foo(x: u8) -> u8 {
//     2 * x
// }

// https://stackoverflow.com/questions/54208702/embedding-actix-web-into-a-struct-so-that-i-can-start-stop-server

pub struct WebService2
{

}

impl WebService2
{
    #[actix_web::main]
    pub async fn start_webserver(self) {
        let server_config = config::get_config();

        HttpServer::new(|| {
            App::new()
                .service(order_handler::order_list)
                .service(
                    web::resource("/order/create")
                        .route(web::post().to(order_handler::order_create)),
                )
                .service(
                    web::resource("/order/cancel")
                        .route(web::post().to(order_handler::order_cancel)),
                )
                .service(
                    web::resource("/order/update")
                        .route(web::post().to(order_handler::order_update)),
                )
                .service(
                    web::resource("/order/fulfill")
                        .route(web::post().to(order_handler::fulfill_order)),
                )
                .service(
                    web::resource("/product/create")
                        .route(web::post().to(product_handler::product_create)),
                )
                .service(
                    web::resource("/product/delete")
                        .route(web::post().to(product_handler::product_delete)),
                )
                .service(
                    web::resource("/product/update")
                        .route(web::post().to(product_handler::product_update)),
                )
                .service(
                    web::resource("/stock/create")
                        .route(web::post().to(stock_handler::stock_create)),
                )
                .service(
                    web::resource("/stock/delete")
                        .route(web::post().to(stock_handler::stock_delete)),
                )
                .service(
                    web::resource("/stock/update")
                        .route(web::post().to(stock_handler::stock_update)),
                )
                .service(
                    web::resource("/stock/increment")
                        .route(web::post().to(stock_handler::stock_increment)),
                )
        })
        .bind(server_config.unwrap())?;

        self.server.run().await
    }
}

// async fn example() {
//     let s = S { foo };
// }

// trait WebService {
//     fn start_service() -> Option<Service>;

//     fn stop_service() -> bool;

//     fn create_service() -> App;
// }

// #[derive(Clone)]
// pub struct WebService {
//     server: HttpServer<App, fn() -> App>,
//     name: String,
//     services: Vec<String>,
// }

// impl MainWebService {
//     pub fn new(name: &String) -> MainWebService {
//         MainWebService {
//             name: name.clone(),
//             service_type: ServiceType::Stateless,
//             args: Vec::new(),
//             dir: None,
//         }
//     }

//     /// Set the type of Microservice
//     pub fn service_type<'a>(&'a mut self, service_type: ServiceType) -> &'a mut Microservice {
//         self.service_type = service_type;
//         self
//     }

//     /// Add settings for the WebService
//     pub fn arg<'a>(&'a mut self, arg: String) -> &'a mut Microservice {
//         self.args.push(arg);
//         self
//     }

//     /// Set the working directory for the web service
//     pub fn run_location<'a>(&'a mut self, dir: String) -> &'a mut Microservice {
//         self.dir = Some(dir);
//         self
//     }

//     /// Executes the command as a child process, which is returned.
//     pub fn start(&self) -> Result<Service, ()> {
//         Ok(Service::new(&self.name))
//     }
// }
