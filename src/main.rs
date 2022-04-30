use actix_web::{App, HttpServer};
use crosstown_bus::Bus;
use tracing_subscriber;
use crate::entity::{CreatedBookEvent, Health};

mod entity;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8092".to_string());
    let address = format!("127.0.0.1:{}", port);

    tracing::info!("Starting server on {}", address);
    receive_created_book_event().await;

    HttpServer::new(move || {
        App::new()
            .service(api::hello)
            .service(api::health)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("🔥🔥🔥 Couldn't start the server in port {}: {:?}", port, err)
    })
    .run()
    .await
}

async fn receive_created_book_event() {
    let url = std::env::var("RABBIT_URL");
    let bus = Bus::new_rabbit_bus(url.unwrap());
    let _ = bus.subscribe_event::<CreatedBookEvent>(String::from("send_email"),|event| {
        tracing::info!("Receive created book event, book_id: {}, title: {}", event.id, event.title);
        (false, Ok(()))
    });
}

