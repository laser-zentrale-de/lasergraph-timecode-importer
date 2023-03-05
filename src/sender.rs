use crate::csv::Entry;
use crate::error::InputError;
use log::{debug, info, trace};
use regex::Regex;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;

fn replace_timestamp_last_colon_to_comma(input: &str) -> Result<String, Box<dyn Error>> {
    let split = input.split(':').collect::<Vec<&str>>();

    if split.len() != 4 {
        return Err("Invalid input".into());
    }

    let timestamp: String = format!("{}:{}:{},{}", split[0], split[1], split[2], split[3]);
    trace!("Timestamp conversion -> Old: {} New: {}", input, timestamp);

    Ok(timestamp)
}

fn format_timestamp(input: &str) -> Result<String, Box<dyn Error>> {
    let timestamp: String;

    // Timestamp schema for Lasergraph DSP
    let regex_timestamp = Regex::new(r"^\d{1,2}:\d{2}:\d{2},\d{2}$")?;

    // Timestamp from Reaper
    let regex_reaper = Regex::new(r"^\d{1,2}:\d{2}:\d{2}:\d{2}$")?;

    // Match for known timestamp formats
    if regex_timestamp.is_match(input) {
        trace!("Timestamp format correct (00:00:00,00)");
        timestamp = input.to_string();
    } else if regex_reaper.is_match(input) {
        trace!("Timestamp is in wrong format (00:00:00:00)");
        timestamp = replace_timestamp_last_colon_to_comma(input)?;
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
        let timestamp: String = format_timestamp(&entry.start)?;
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
    use std::net::TcpListener;

    #[test]
    fn test_replace_timestamp_last_colon_to_comma() {
        // Should be Ok()
        assert_eq!(
            replace_timestamp_last_colon_to_comma("00:00:00:00").unwrap(),
            "00:00:00,00".to_string()
        );
        assert_eq!(
            replace_timestamp_last_colon_to_comma("05:14:46:24").unwrap(),
            "05:14:46,24".to_string()
        );
        assert_eq!(
            replace_timestamp_last_colon_to_comma("1:35:22:05").unwrap(),
            "1:35:22,05".to_string()
        );

        // Should be Err()
        assert!(replace_timestamp_last_colon_to_comma("1;35:22:05").is_err());
        assert!(replace_timestamp_last_colon_to_comma("1:35:22;05").is_err());
        assert!(replace_timestamp_last_colon_to_comma("1:35:22-05").is_err());
    }

    #[test]
    fn test_format_timestamp() {
        // Should be Ok()
        assert_eq!(
            format_timestamp("00:00:00:00").unwrap(),
            "00:00:00,00".to_string()
        );
        assert_eq!(
            format_timestamp("05:14:46:24").unwrap(),
            "05:14:46,24".to_string()
        );
        assert_eq!(
            format_timestamp("1:35:22:05").unwrap(),
            "1:35:22,05".to_string()
        );

        // Should be Err()
        assert!(format_timestamp("1;35:22:05").is_err());
        assert!(format_timestamp("1:35:22;05").is_err());
        assert!(format_timestamp("1:35:22-05").is_err());
    }

    #[test]
    fn test_send_entries() -> Result<(), Box<dyn Error>> {
        // Set up a mock server
        let listener = TcpListener::bind("127.0.0.1:8210")?;
        let server_addr = listener.local_addr()?;
        let mut stream = TcpStream::connect(server_addr)?;
        let (mut incoming, _) = listener.accept()?;
        let expected_packet = "test packet\n";

        // Send the test packet
        send_tcp_packet(&mut stream, "test packet")?;

        // Read the received packet
        let mut buf = [0; 1024];
        let n = incoming.read(&mut buf)?;

        // Convert bytes to string
        let received_packet = String::from_utf8_lossy(&buf[..n]);

        assert_eq!(expected_packet, received_packet);

        Ok(())
    }
}
