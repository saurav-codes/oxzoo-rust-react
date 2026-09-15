use std::net::SocketAddr;

use axum::http::{header::CONTENT_TYPE, HeaderValue};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{extract::State, Router};

// PORT is injected by the ox platform environment file; the process command
// never carries a port.
#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid port number");
    let greeting_line = format!(
        "hello world oxzoo-rust-react_{}",
        std::env::var("GREETING_TAG").expect("GREETING_TAG must be set")
    );

    let app = Router::new()
        .route("/api/greeting", get(greeting))
        .route("/health", get(health))
        .with_state(greeting_line);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    println!("listening on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}

async fn greeting(State(greeting): State<String>) -> Response {
    let mut response = greeting.into_response();
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    response
}

async fn health() -> &'static str {
    "ok"
}
