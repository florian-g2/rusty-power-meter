use std::sync::Arc;
use axum::http::header;
use axum::response::Response;
use serde::Serialize;
use crate::database::ReadonlyDatabase;




pub async fn handler(database: Arc<ReadonlyDatabase>, body: String) -> Response {
    let result = database.query(&body);
    match result {
        Ok(query_result) => {
            let json = serde_json::to_string(&query_result).unwrap();
            
            Response::builder()
                .status(200)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json.into())
                .unwrap()
        }
        Err(error) => {
            Response::builder()
                .status(400)
                .body(format!("{{\"error\": \"{}\"}}", error.to_string()).into())
                .unwrap()
        }
    }
}