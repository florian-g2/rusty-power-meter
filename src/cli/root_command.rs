use clap_derive::{Parser, Subcommand};
use crate::cli::database::DatabaseCommand;
use crate::cli::ports::ListPortsCommand;
use crate::cli::start::StartCommand;

/// Rusty Power Meter - Copyright (c) 2024 Florian Gäbler
#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct RootCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    Database(DatabaseCommand),
    ListPorts(ListPortsCommand),
    Start(StartCommand),
}

impl RootCommand {
    pub fn run(self) -> Result<(), anyhow::Error> {
        match self.command {
            Commands::Database(command) => command.run(),
            Commands::ListPorts(command) => command.run(),
            Commands::Start(command) => command.run(),
        }
    }
}