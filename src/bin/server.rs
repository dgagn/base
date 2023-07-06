use base::build_app;
use base::telemetry;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let _guard = telemetry::init();

    let app = build_app();

    let addr = [127, 0, 0, 1];
    let port = 8000;
    let full_addr: SocketAddr = (addr, port).into();

    tracing::debug!("Listening on {}", full_addr);
    axum::Server::bind(&full_addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind server");
}
