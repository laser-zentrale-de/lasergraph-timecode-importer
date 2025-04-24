mod cli;
mod csv;
mod error;
mod sender;

use crate::csv::Entry;
use clap::{CommandFactory, Parser};
use log::{error, info};
use std::path::PathBuf;

fn main() {
    // Initialize the logger
    env_logger::init();

    info!("lasergraph-dsp-timecode-importer started");

    // Parse arguments from CLI
    let args = cli::Args::parse();

    match args.command {
        // Handle shell completions
        Some(cli::Commands::Completions { shell }) => {
            let mut cmd = cli::Args::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
        }

        // Run the importer
        Some(cli::Commands::Import {
            address,
            port,
            csv,
            create_entry,
            start_entry,
        }) => {
            let target: String = format!("{}:{}", address, port);
            let filepath: PathBuf = csv;
            let entry_offset: i32 = start_entry;

            // Get entries from CSV
            let entries: Vec<Entry> = match csv::get_csv_entries(filepath) {
                Ok(parsed_entries) => parsed_entries,
                Err(e) => {
                    error!("Failed to parse entries from CSV file with error: {}", e);
                    std::process::exit(1);
                }
            };

            // Send entries to DSP
            match sender::send(&target, entries, create_entry, entry_offset) {
                Ok(()) => println!("Successfully sent entries to DSP {}", target),
                Err(e) => {
                    error!("Failed to send entries to DSP with error: {}", e);
                    std::process::exit(1);
                }
            };
        }

        // Show help if no subcommand is provided
        None => {
            cli::Args::command().print_help().unwrap();
            std::process::exit(0);
        }
    }
}
