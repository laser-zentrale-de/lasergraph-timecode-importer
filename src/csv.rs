use std::fs::File;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Entry {
    number: String,
    name: String,
    start: String,
}

pub fn get_csv_entries(filepath: &str) -> Result<Vec<Entry>, Box<dyn Error>> {

    // Open file
    let file = File::open(filepath)?;

    let mut rdr = csv::Reader::from_reader(file);

    let mut entries = Vec::new();

    for result in rdr.deserialize() {
        let record: Entry = result?;
        println!("{:?}", record);
        entries.push(record);
    }

    Ok(entries)
}