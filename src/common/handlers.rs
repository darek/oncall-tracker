use axum::{http::StatusCode, response::IntoResponse};

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "page not found")
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "toodle-oo"
}
