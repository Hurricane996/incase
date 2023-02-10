use std::error::Error;

mod video_pull;
mod auth;

use actix_web::{App,HttpServer, Responder, get, HttpResponse, web};
use auth::auth_service;
use video_pull::video_service;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_string = std::env::var("POSTGRES_CONNECTION_STRING").expect("No postgres connection string provided");
    std::env::set_var("RUST_LOG", "debug");
    //std::env::set_var("RUST_BACKTRACE", "full");

    env_logger::init();
    
    HttpServer::new(move || {
        App::new()
            .service(auth_service("auth"))
            .service(video_service(
                "localhost:8080",
                "video",
                connection_string.clone()
            ))
            //.default_service(web::to(e404))
            .service(hello_world)
    }).bind("0.0.0.0:8080")?
    .run().await?;

    Ok(())
}

async fn e404() -> impl Responder {
   let mut r = HttpResponse::NotFound();
   r.content_type("text/html");
   r.body("<h1>File not found</h1>");
   r
}

#[get("/")]
async fn hello_world() -> impl Responder {
    "Hello, world!"
}
