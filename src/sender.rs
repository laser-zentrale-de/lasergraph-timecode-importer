use std::io::prelude::*;
use std::error::Error;
use std::net::{TcpStream};

fn send_tcp_packet(stream: &mut TcpStream, packet:&str) -> Result<(), Box<dyn Error>> {
    // Simulate ENTER with the additional \n
    let command = format!("{}\n", packet);

    // Write to the stream
    stream.write_all(command.as_bytes())?;

    // Output the packet without \n
    println!("Sent packet: {}", packet);

    Ok(())
}

pub fn send_entries(target: &str) -> Result<(), Box<dyn Error>> {
    // Open TCP/IP stream
    let mut stream = TcpStream::connect(target)?;

    // swtich to main windows
    send_tcp_packet(&mut stream, "root")?;

    // Add entry 1 to film 2
    send_tcp_packet(&mut stream, "edit")?;
    send_tcp_packet(&mut stream, "film2")?;
    send_tcp_packet(&mut stream, "insert entry 1")?;
    send_tcp_packet(&mut stream, "")?;

    // Add entry 1 to timescript
    send_tcp_packet(&mut stream, "script1")?;
    send_tcp_packet(&mut stream, "insert 1:00:17,04 entry 1")?;
    send_tcp_packet(&mut stream, "")?;

    // Exit
    send_tcp_packet(&mut stream, "exit")?;

    // Exit TCP/IP stream
    drop(stream);

    Ok(())
}
