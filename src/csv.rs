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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    #[test]
    fn test_get_csv_entries() {
        // Create a temporary file for testing
        let file_content = "\
            #,Name,Start\n\
            M1,Vocals,00:00:01:23\n\
            M2,First Drop,1:03:55:02\n\
            M3,Second Drop,10:43:20:01\n\
        ";
        let tmp_dir = tempfile::tempdir().unwrap();
        let file_path = tmp_dir.path().join("test.csv");
        std::fs::write(&file_path, file_content).unwrap();

        // Call the function under test
        let result = get_csv_entries(file_path);

        // Check the result
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].number, "M1");
        assert_eq!(entries[0].name, "Vocals");
        assert_eq!(entries[0].start, "00:00:01:23");
        assert_eq!(entries[1].number, "M2");
        assert_eq!(entries[1].name, "First Drop");
        assert_eq!(entries[1].start, "1:03:55:02");
        assert_eq!(entries[2].number, "M3");
        assert_eq!(entries[2].name, "Second Drop");
        assert_eq!(entries[2].start, "10:43:20:01");
    }
}
