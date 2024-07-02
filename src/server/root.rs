use axum::http::header;
use axum::response::Response;

pub async fn get_handler() -> Response {
    let help_text = "
        Service is running.

        GET /now - get the latest meter reading
        GET /api/now - get the latest meter reading as JSON
        POST /api/query - query the database with readonly SQLite statements
    ";
    
    Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(help_text.into())
        .unwrap()
}