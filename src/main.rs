use anyhow::Error;
use clap::Parser;

use crate::cli::root_command::RootCommand;

mod obis_code;
mod unit;
mod meter_reading;
mod cli;
mod database;
mod core_loop;
mod server;

fn main() -> Result<(), Error> { RootCommand::parse().run() }