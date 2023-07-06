pub mod email;
pub mod telemetry;

use axum::Router;
use axum::{
    extract::MatchedPath,
    http::{Request, StatusCode},
    routing::get,
};
use axum_macros::debug_handler;
use tower_http::trace::TraceLayer;
use tracing::info_span;

pub fn build_app() -> Router {
    Router::new()
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
        )
}

#[debug_handler]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
