use std::time::Duration;
use serialport::{Parity, StopBits};
use crate::database::Database;
use crate::meter_reading::MeterReading;
use std::io::{Read};
use std::sync::{Arc};
use anyhow::Error;
use crossbeam_utils::atomic::AtomicCell;

pub struct CoreLoop<'a> { 
    port: String,
    database: &'a Database,
    latest_reading: Arc<AtomicCell<Option<MeterReading>>>,
    verbose: bool
}

impl<'a> CoreLoop<'a> {
    pub fn new(port: String, verbose: bool, database: &'a Database) -> Self {
        Self {
            port,
            database,
            latest_reading: Arc::new(AtomicCell::new(None)),
            verbose
        }
    }

    pub fn enter(&self) -> Result<(), Error> {
        let port = serialport::new(&self.port, 9_600)
            .stop_bits(StopBits::One)
            .parity(Parity::None)
            .timeout(Duration::from_millis(5000))
            .open()
            .expect("Failed to open port");

        
        // let mut current_ball_position = 1;
        let mut decoder = sml_rs::transport::Decoder::<Vec<u8>>::new();
        
        println!("Now listening for SML messages on {}...", self.port);

        for res in port.bytes() {
            let byte = res?;

            match decoder.push_byte(byte) {
                Ok(None) => {}
                Ok(Some(decoded_bytes)) => {
                    let result = sml_rs::parser::complete::parse(decoded_bytes);
                    let Ok(sml_file) = result else {
                        if self.verbose {
                            println!("Err({:?})", result);
                        }
                        continue;
                    };

                    let reading = MeterReading::parse(sml_file);
                    let Ok(reading) = reading else {
                        continue;
                    };

                    // println!("{}", reading.display_compact());
                    if self.verbose {
                        println!("{}", reading.display_compact());
                    }
                    
                    self.database.insert_reading(&reading)?;
                    self.latest_reading.store(Some(reading));
                    
                    
                    // print_progress_bar(&mut current_ball_position);
                }
                Err(e) => {
                    if self.verbose {
                        println!("Err({:?})", e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn get_latest_reading_cell(&self) -> Arc<AtomicCell<Option<MeterReading>>> {
        self.latest_reading.clone()
    }
}