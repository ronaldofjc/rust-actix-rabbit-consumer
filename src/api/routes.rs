use actix_web::{HttpResponse, get};
use crate::{Health};

#[get("/")]
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("Consumer Events with Rust and Actix Web it's working!")
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(Health { status: "Ok".to_string() })
}