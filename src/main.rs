/* Simple Rust backend */


// Crates
use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpResponse, Result};
use log::debug;
use env_logger;
use std::path::PathBuf;

// Logger Struct
struct LoggerCalls;

impl LoggerCalls {
    fn debug_call(&self, message: &str) {
        debug!("{}", message);
    }
}

async fn home() -> Result<NamedFile> {
    let lc = LoggerCalls;
    lc.debug_call("Landing page called");
    Ok(NamedFile::open("html/index.html")?)
}

async fn about_us() -> Result<NamedFile> {
    let lc = LoggerCalls;
    lc.debug_call("About us page called");
    Ok(NamedFile::open("html/about_us.html")?)
}

async fn join_us() -> Result<NamedFile> {
    let lc = LoggerCalls;
    lc.debug_call("Join us page called");
    Ok(NamedFile::open("html/join_us.html")?)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
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
