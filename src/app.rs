use std::env;

use axum::{routing::get, Router};

pub async fn run() {
    let app = Router::new().route("/api/healthz", get(|| async {}));

    let addr = &env::var("APP_ADDR").unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Axum started");
    axum::serve(listener, app).await.unwrap();
}
