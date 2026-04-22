use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "DooDoo Logistics Rust API is alive"
}