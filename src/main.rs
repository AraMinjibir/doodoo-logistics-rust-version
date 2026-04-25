use actix_web::{get, App, HttpServer, Responder};

mod domain;
mod repositories;
mod infrastructure;

#[get("/health")]
async fn health() -> impl Responder {
    "DooDoo Logistics (Actix) is alive"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}