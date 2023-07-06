mod email;
mod telemetry;
use std::net::SocketAddr;

use axum::Router;
use axum::{
    extract::MatchedPath,
    http::{Request, StatusCode},
    routing::get,
};
use email::{Email, EmailClient};
use tower_http::trace::TraceLayer;
use tracing::info_span;

#[tokio::main]
async fn main() {
    let _guard = telemetry::init();

    let email_client = EmailClient::new();
    let email = Email {
        to: "ddanygagnon@gmail.com",
        subject: "Hello from Rust",
        body: "Hello from Rust",
    };

    email_client.send(&email).unwrap();

    let app: Router = Router::new()
        .route("/health-check", get(health_check))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    let addr = [127, 0, 0, 1];
    let port = 8000;
    let full_addr: SocketAddr = (addr, port).into();

    tracing::debug!("Listening on {}", full_addr);
    axum::Server::bind(&full_addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind server");
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
