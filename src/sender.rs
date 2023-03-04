use crate::csv::Entry;
use crate::error::InputError;
use log::{debug, info, trace};
use regex::Regex;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;

fn replace_timestamp_colon_to_comma(input: &str) -> String {
    let split = input.split(':').collect::<Vec<&str>>();
    let timestamp: String = format!("{}:{}:{},{}", split[0], split[1], split[2], split[3]);
    trace!("Timestamp conversion -> Old: {} New: {}", input, timestamp);

    timestamp
}

fn get_correct_timestamp(input: &str) -> Result<String, Box<dyn Error>> {
    let timestamp: String;

    // Timestamp schema for Lasergraph DSP
    let regex_timestamp = Regex::new(r"^\d{1,2}:\d{2}:\d{2},\d{2}$")?;

    // Timestamp from Reaper
    let regex_reaper = Regex::new(r"^\d{1,2}:\d{2}:\d{2}:\d{2}$")?;

    if regex_timestamp.is_match(input) {
        trace!("Timestamp format correct (00:00:00,00)");
        timestamp = input.to_string();
    } else if regex_reaper.is_match(input) {
        trace!("Timestamp is in wrong format (00:00:00:00)");
        timestamp = replace_timestamp_colon_to_comma(input);
    } else {
        debug!("Timestamp {} does not match any known format", input);

        return Err(Box::new(InputError::ParseError(
            "Time stamp does not match any known schema".to_string(),
        )));
    }

    Ok(timestamp)
}

fn send_tcp_packet(stream: &mut TcpStream, packet: &str) -> Result<(), Box<dyn Error>> {
    // Simulate ENTER with the additional \n
    let command = format!("{}\n", packet);

    // Write to the stream
    stream.write_all(command.as_bytes())?;

    // Output the packet without \n
    debug!("Sent packet to DSP: {}", packet);

    Ok(())
}

pub fn send_entries(
    target: &str,
    entries: Vec<Entry>,
    entry_offset: i32,
) -> Result<(), Box<dyn Error>> {
    // Open TCP/IP stream
    let mut stream = TcpStream::connect(target)?;
    info!("TCP stream opened to address: {}", target);

    // swtich to DSP main window
    send_tcp_packet(&mut stream, "root")?;

    // Loop through entries
    let mut i: i32 = entry_offset;
    for entry in entries {
        // Convert timestamp
        let timestamp: String = get_correct_timestamp(&entry.start)?;
        let timescript_insert: String = format!("insert {} entry {}", timestamp, i);

        // Convert Count variable
        let entry_insert: String = format!("insert entry {}", i);

        // Add entry 1 to film 1
        send_tcp_packet(&mut stream, "edit")?;
        send_tcp_packet(&mut stream, "film1")?;
        send_tcp_packet(&mut stream, &entry_insert)?;
        send_tcp_packet(&mut stream, "")?;

        // Add entry 1 to timescript
        send_tcp_packet(&mut stream, "script1")?;
        send_tcp_packet(&mut stream, &timescript_insert)?;
        send_tcp_packet(&mut stream, "")?;

        i += 1;
    }

    // Exit
    send_tcp_packet(&mut stream, "root")?;

    // Exit TCP/IP stream
    drop(stream);
    info!("TCP stream closed to address: {}", target);

    Ok(())
}

// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_replace_timestamp_colon_to_comma() {
        assert_eq!(
            replace_timestamp_colon_to_comma("00:00:00:00"),
            "00:00:00,00"
        );
        assert_eq!(
            replace_timestamp_colon_to_comma("05:14:46:24"),
            "05:14:46,24"
        );
        assert_eq!(replace_timestamp_colon_to_comma("1:35:22:05"), "1:35:22,05");
    }

    #[test]
    #[should_panic]
    fn test_panic_replace_timestamp_colon_to_comma_01() {
        assert_eq!(
            replace_timestamp_colon_to_comma("00.00:00:00"),
            "00:00:00,00"
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_replace_timestamp_colon_to_comma_02() {
        assert_eq!(
            replace_timestamp_colon_to_comma("05;14:46:24"),
            "05:14:46,24"
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_replace_timestamp_colon_to_comma_03() {
        assert_eq!(replace_timestamp_colon_to_comma("1:35:22-05"), "1:35:22,05");
    }
}
