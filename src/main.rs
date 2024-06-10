/* Crates */
use actix_web::{web, App, HttpServer, HttpResponse};
use log::{debug, info};
use std::io;

// Logger Struct
struct LoggerCalls;

impl LoggerCalls {
    fn debug_call(&self, message: &str) {
        debug!("{}", message);
    }
}

async fn home() -> HttpResponse {
    let lc = LoggerCalls;
    lc.debug_call("Landing page called");
    HttpResponse::Ok().body("Hello world!")
}

async fn about_us() -> HttpResponse {
    let lc = LoggerCalls;
    lc.debug_call("About us page called");
    HttpResponse::Ok().body("About us page")
}

async fn join_us() -> HttpResponse {
    let lc = LoggerCalls;
    lc.debug_call("Join us page called");
    HttpResponse::Ok().body("Join us page")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(home))
            .service(web::resource("/about_us").to(about_us))
            .service(web::resource("/join_us").to(join_us))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
