pub mod handlers;
pub mod notifications;
pub mod telemetry;

use axum::Router;
use axum::{extract::MatchedPath, http::Request, routing::get};
use handlers::*;
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
