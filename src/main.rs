use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(HelloResponse {
        message: "Hello, world!".to_owned(),
    })
}

#[post("/echo")]
async fn echo(body: web::Json<HelloResponse>) -> impl Responder {
    HttpResponse::Ok().json(body.into_inner())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
}
