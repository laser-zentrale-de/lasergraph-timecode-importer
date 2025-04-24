use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

/// Import timecode timestamps and entries to Lasergraph DSP
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completions
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Run the CSV importer
    Import {
        /// IP-Address of the Lasergraph DSP
        #[arg(short, long, value_name = "IP-ADDRESS")]
        address: String,

        /// TCP/IP port of the lasergraph DSP for remoting
        #[arg(short, long, default_value_t = 8210)]
        port: i32,

        /// Path to the CSV-file
        #[arg(short, long, value_name = "FILE")]
        csv: PathBuf,

        /// Defines if entries should be created
        #[arg(short = 'e', long)]
        create_entry: bool,

        /// Defines the number of the first Entry
        #[arg(short, long, default_value_t = 0, value_name = "ENTRY")]
        start_entry: i32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        // Test importer with only neccessary args
        let args = Args::parse_from([
            "dummy", //argv[0] will be ignored
            "import",
            "--address",
            "127.0.0.1",
            "--csv",
            "/path/to/file.csv",
        ]);

        match args.command {
            Some(Commands::Import {
                address,
                port,
                csv,
                create_entry,
                start_entry,
            }) => {
                assert_eq!(address, "127.0.0.1");
                assert_eq!(csv.to_str(), Some("/path/to/file.csv"));
                assert_eq!(port, 8210);
                assert!(!create_entry);
                assert_eq!(start_entry, 0);
            }
            _ => {
                panic!("Expteced 'import' command")
            }
        }

        // Test importer with all arguments
        let args = Args::parse_from([
            "dummy", //argv[0] will be ignored
            "import",
            "--address",
            "127.0.0.1",
            "--csv",
            "/path/to/file.csv",
            "--port",
            "8000",
            "--start-entry",
            "50",
            "--create-entry",
        ]);

        match args.command {
            Some(Commands::Import {
                address,
                port,
                csv,
                create_entry,
                start_entry,
            }) => {
                assert_eq!(address, "127.0.0.1");
                assert_eq!(csv.to_str(), Some("/path/to/file.csv"));
                assert_eq!(port, 8000);
                assert!(create_entry);
                assert_eq!(start_entry, 50);
            }
            _ => {
                panic!("Expected 'import' command")
            }
        }
    }
}
