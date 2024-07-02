use std::thread;
use anyhow::Error;
use clap_derive::{Args};
use crate::core_loop::CoreLoop;
use crate::database::Database;
use crate::server::Server;

#[derive(Clone, Args)]
pub struct StartCommand { 
    #[arg(long)]
    port: String,
    
    #[arg(long, default_value = "false")]
    verbose: bool,
}

impl StartCommand {
    pub fn run(self) -> Result<(), Error> {
        let database = Database::load()?;

        let core_loop = CoreLoop::new(self.port, self.verbose, &database);
        let latest_reading_cell = core_loop.get_latest_reading_cell();
        
        let server_thread = thread::spawn(|| {
            Server::create(3000, latest_reading_cell).enter()
        });
        
        core_loop.enter()?;
        
        server_thread.join().unwrap()?;
        Ok(())
    }
}