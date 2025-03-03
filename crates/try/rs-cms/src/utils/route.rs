use axum::{
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
};

// 统一处理404:
pub async fn handler_404(method: Method, uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Oops! 404\n\nNothing to see at {} {}", method, uri))
}
