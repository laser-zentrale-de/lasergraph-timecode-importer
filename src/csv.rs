use log::debug;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "#")]
    pub number: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Start")]
    pub start: String,
}

pub fn get_csv_entries(filepath: std::path::PathBuf) -> Result<Vec<Entry>, Box<dyn Error>> {
    // Open file
    let file = File::open(filepath)?;

    // Read from file
    let mut rdr = csv::Reader::from_reader(file);

    // Deserialize csv and add to new vector
    let mut entries = Vec::new();

    for result in rdr.deserialize() {
        let record: Entry = result?;
        debug!("Parsed csv: {:?}", record);
        entries.push(record);
    }

    Ok(entries)
}
