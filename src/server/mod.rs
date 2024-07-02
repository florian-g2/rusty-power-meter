mod api;
mod root;
mod now;

use std::io;
use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crossbeam_utils::atomic::AtomicCell;
use crate::database::ReadonlyDatabase;
use crate::meter_reading::MeterReading;

pub struct Server {
    app: Router,
    port: u16
}

impl Server {
    pub fn create(port: u16, latest_reading_cell: Arc<AtomicCell<Option<MeterReading>>>) -> Self {
        let latest_reading_cell = (
            latest_reading_cell.clone(),
            latest_reading_cell.clone()
        );
        
        let readonly_database = Arc::new(ReadonlyDatabase::load().unwrap());
        
        // build our application with a single route
        let app = Router::new()
            .route("/", get(root::get_handler))
            .route("/now", get(move || now::handler(latest_reading_cell.0.clone())))
            .route("/api/now", get(move || api::now::handler(latest_reading_cell.1.clone())))
            .route("/api/query", post(move |body: String| api::query::handler(readonly_database.clone(), body)));

        Server {
            app,
            port
        }
    }

    pub fn enter(self) -> io::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap()
            .block_on(async move {
                // let addr = format!("127.0.0.1:{}", self.port);
                let addr = format!("0.0.0.0:{}", self.port);
                let listener = tokio::net::TcpListener::bind(addr).await?;
                
                let future = axum::serve(listener, self.app);
                println!("Now listening for HTTP requests on TCP port {}...", self.port);
                
                future.await
            })
    }
}