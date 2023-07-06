extern crate base;
use base::telemetry;
use std::net::{SocketAddr, TcpListener};

mod handlers;

pub async fn spawn_app() -> SocketAddr {
    let _guard = telemetry::init();
    let app = base::build_app();
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let _ = tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    addr
}
