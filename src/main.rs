mod csv;
mod sender;

use crate::csv::Entry;
use clap::Parser;

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
    // Parse arguments from CLI
    let args = Args::parse();

    let target: String = format!("{}:{}", args.address, args.port.to_string());
    let filepath: std::path::PathBuf = args.csv;
    let entry_offset: i32 = args.start;

    // Create Entry result vector
    let entries: Vec<Entry>;

    // Get entries from CSV
    match csv::get_csv_entries(filepath) {
        Ok(parsed_entries) => {
            entries = parsed_entries;
        }
        Err(e) => {
            eprintln!("Failed to parse entries from CSV file.\nError: {}", e);
            std::process::exit(1);
        }
    }

    // Send entries to DSP
    match sender::send_entries(&target, entries, entry_offset) {
        Ok(()) => println!("Successfully imported entries to DSP {}", target),
        Err(e) => {
            eprintln!("Failed to import entries to DSP {}\nError: {}", target, e);
            std::process::exit(1);
        }
    }
}
