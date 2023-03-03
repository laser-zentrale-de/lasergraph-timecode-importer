use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;

use crate::csv::Entry;
use log::{debug, trace};

fn replace_timestamp_colon_to_comma(input: &str) -> String {
    let split = input.split(':').collect::<Vec<&str>>();
    let timestamp: String = format!("{}:{}:{},{}", split[0], split[1], split[2], split[3]);
    trace!("Timestamp conversion -> Old: {} New: {}", input, timestamp);

    timestamp
}

fn send_tcp_packet(stream: &mut TcpStream, packet: &str) -> Result<(), Box<dyn Error>> {
    // Simulate ENTER with the additional \n
    let command = format!("{}\n", packet);

    // Write to the stream
    stream.write_all(command.as_bytes())?;

    // Output the packet without \n
    debug!("Sent packet: {}", packet);

    Ok(())
}

pub fn send_entries(
    target: &str,
    entries: Vec<Entry>,
    entry_offset: i32,
) -> Result<(), Box<dyn Error>> {
    // Open TCP/IP stream
    let mut stream = TcpStream::connect(target)?;

    // swtich to DSP main window
    send_tcp_packet(&mut stream, "root")?;

    // Loop through entries
    let mut i: i32 = entry_offset;
    for entry in entries {
        // Convert timestamp
        let start: String = entry.start;
        let timestamp: String = replace_timestamp_colon_to_comma(&start);
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

    Ok(())
}
