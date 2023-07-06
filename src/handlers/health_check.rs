use axum::http::StatusCode;
use axum_macros::debug_handler;

#[debug_handler]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
