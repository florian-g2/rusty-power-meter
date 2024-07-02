use anyhow::Error;
use clap_derive::{Args};
use crate::database::Database;

#[derive(Clone, Args)]
pub struct DatabaseCommand { }

impl DatabaseCommand {
    pub fn run(self) -> Result<(), Error> {
        let db = Database::load()?;
        let metrics = db.metrics()?;
        
        println!("{metrics}");
        
        Ok(())
    }
}