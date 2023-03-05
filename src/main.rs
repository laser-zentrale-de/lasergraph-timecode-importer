mod csv;
mod error;
mod sender;
mod cli;

use crate::csv::Entry;
use clap::Parser;
use log::{error, info};
use std::path::{PathBuf};

fn main() {
    // Initialize the logger
    env_logger::init();

    info!("lasergraph-dsp-timecode-importer started");

    // Parse arguments from CLI
    let args = cli::Args::parse();

    let target: String = format!("{}:{}", args.address, args.port);
    let filepath: PathBuf = args.csv;
    let entry_offset: i32 = args.start;

    // Get entries from CSV
    let entries: Vec<Entry> = match csv::get_csv_entries(filepath) {
        Ok(parsed_entries) => parsed_entries,
        Err(e) => {
            error!("Failed to parse entries from CSV file with error: {}", e);
            std::process::exit(1);
        }
    };

    // Send entries to DSP
    match sender::send_entries(&target, entries, entry_offset) {
        Ok(()) => println!("Successfully sent entries to DSP {}", target),
        Err(e) => {
            error!("Failed to send entries to DSP with error: {}", e);
            std::process::exit(1);
        }
    };
}
