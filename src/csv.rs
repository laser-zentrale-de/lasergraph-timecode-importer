use log::debug;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub number: String,
    pub name: String,
    pub start: String,
}

pub fn get_csv_entries(filepath: std::path::PathBuf) -> Result<Vec<Entry>, Box<dyn Error>> {
    // Open file
    let file = File::open(filepath)?;

    let mut rdr = csv::Reader::from_reader(file);

    let mut entries = Vec::new();

    for result in rdr.deserialize() {
        let record: Entry = result?;
        debug!("Parsed csv: {:?}", record);
        entries.push(record);
    }

    Ok(entries)
}
