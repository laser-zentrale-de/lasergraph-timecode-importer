mod csv;
mod error;
mod sender;

use crate::csv::Entry;
use clap::Parser;
use log::{error, info};

/// Import timecode entries to Lasergraph DSP
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP-Address of the Lasergraph DSP
    #[arg(short, long)]
    address: String,

    /// TCP/IP port of the lasergraph DSP for remoting
    #[arg(short, long, default_value_t = 8210)]
    port: i32,

    /// Path to the CSV-file
    #[arg(short, long)]
    csv: std::path::PathBuf,

    /// Start number of the Entries that should be created
    #[arg(short, long, default_value_t = 0)]
    start: i32,
}

fn main() {
    // Initialize the logger
    env_logger::init();

    info!("lasergraph-dsp-timecode-importer started");

    // Parse arguments from CLI
    let args = Args::parse();

    let target: String = format!("{}:{}", args.address, args.port);
    let filepath: std::path::PathBuf = args.csv;
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
