use anyhow::Error;
use clap_derive::{Args};

#[derive(Clone, Args)]
pub struct ListPortsCommand { }

impl ListPortsCommand {
    pub fn run(self) -> Result<(), Error> {
        let ports = serialport::available_ports();
        
        match ports {
            Ok(ports) => {
                if ports.is_empty() {
                    println!("No ports available.");
                }
                
                for port in ports {
                    println!("{:?}", port);
                }
            },
            Err(e) => {
                println!("Could not fetch available ports: {:?}", e);
            }
        }
        
        Ok(())
    }
}