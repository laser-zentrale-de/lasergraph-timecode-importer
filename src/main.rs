mod csv;
mod sender;

use crate::csv::Entry;

fn main() {

    // Define input variabes - later user input via cli
    let target: &str = "192.168.55.100:8210";
    let filepath: &str = "/home/dstrobel/Documents/entries.csv";

    let mut entries: Vec<Entry> = Vec::new();

    // Get entries from CSV
    match csv::get_csv_entries(filepath) {
        Ok(parsed_entries) => {
            entries = parsed_entries;
        },
        Err(e) => eprintln!("Failed to parse entries from CSV file {}\nError: {}", filepath, e),
    }

    // Get entries from CSV
    match sender::send_entries(target, entries) {
        Ok(()) => println!("Successfully imported entries to DSP {}", target),
        Err(e) => eprintln!("Failed to import entries to DSP {}\nError: {}", target, e),
    }
}
