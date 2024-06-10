/* Crates */
use actix_web::{web, App, HttpServer, HttpResponse};
use log::debug;
use std::io;

// Logger Struct
struct LoggerCalls;

// Function implementation for logger struct
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
    // Initialize the logger
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/about_us", web::get().to(about_us))
            .route("/join_us", web::get().to(join_us))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

    .bind("127.0.0.1:8080")?
    .run()
    .await
}
