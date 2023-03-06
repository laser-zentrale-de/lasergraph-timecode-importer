use clap::Parser;
use std::path::PathBuf;

/// Import timecode timestamps and entries to Lasergraph DSP
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
    pub struct Args {
    /// IP-Address of the Lasergraph DSP
    #[arg(short, long, value_name = "IP-ADDRESS")]
    pub address: String,

    /// TCP/IP port of the lasergraph DSP for remoting
    #[arg(short, long, default_value_t = 8210)]
    pub port: i32,

    /// Path to the CSV-file
    #[arg(short, long, value_name = "FILE")]
    pub csv: PathBuf,

    /// Defines if entries should be created
    #[arg(short = 'e', long)]
    pub create_entry: bool,

    /// Defines the number of the first Entry
    #[arg(short, long, default_value_t = 0, value_name = "ENTRY")]
    pub start_entry: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        // Test with only neccessary args
        let args = Args::parse_from(&[
            "program_name",
            "--address",
            "127.0.0.1",
            "--csv",
            "/path/to/file.csv",
        ]);

        assert_eq!(args.address, "127.0.0.1");
        assert_eq!(args.csv.to_str(), Some("/path/to/file.csv"));
        assert_eq!(args.port, 8210);
        assert_eq!(args.start_entry, 0);

        // Test with port
        let args = Args::parse_from(&[
            "program_name",
            "--address",
            "127.0.0.1",
            "--csv",
            "/path/to/file.csv",
            "--port",
            "8000",
        ]);

        assert_eq!(args.address, "127.0.0.1");
        assert_eq!(args.csv.to_str(), Some("/path/to/file.csv"));
        assert_eq!(args.port, 8000);
        assert_eq!(args.start_entry, 0);

        // Test with start_entry
        let args = Args::parse_from(&[
            "program_name",
            "--address",
            "127.0.0.1",
            "--csv",
            "/path/to/file.csv",
            "--start-entry",
            "50",
        ]);

        assert_eq!(args.address, "127.0.0.1");
        assert_eq!(args.csv.to_str(), Some("/path/to/file.csv"));
        assert_eq!(args.port, 8210);
        assert_eq!(args.start_entry, 50);

        // Test with create_entry
        let args = Args::parse_from(&[
            "program_name",
            "--address",
            "127.0.0.1",
            "--csv",
            "/path/to/file.csv",
            "--create-entry",
        ]);

        assert_eq!(args.address, "127.0.0.1");
        assert_eq!(args.csv.to_str(), Some("/path/to/file.csv"));
        assert_eq!(args.port, 8210);
        assert_eq!(args.create_entry, true);
    }
}
