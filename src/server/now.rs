use std::sync::Arc;

use axum::http::header;
use axum::response::Response;
use crossbeam_utils::atomic::AtomicCell;

use crate::meter_reading::MeterReading;

pub async fn handler(latest_reading_cell: Arc<AtomicCell<Option<MeterReading>>>) -> Response {
    let reading = latest_reading_cell.take();

    let status = if reading.is_some() { 200 } else { 204 };

    let body = match reading {
        Some(reading) => format!("{reading}"),
        None => "".to_string()
    };

    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(body.into())
        .unwrap()
}