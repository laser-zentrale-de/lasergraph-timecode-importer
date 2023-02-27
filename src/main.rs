use std::io::prelude::*;
use std::net::{TcpStream};

fn send_tcp_packet(stream: &mut TcpStream, packet:&str) {
    let command = format!("{}\n", packet);

    stream.write_all(command.as_bytes()).unwrap();

    println!("Sent packet: {}", packet);
}

fn main() {
    let target = "192.168.55.100:8210";

    let mut stream = TcpStream::connect(target).unwrap();

    // Add entry 1 to film 2
    send_tcp_packet(&mut stream, "edit");
    send_tcp_packet(&mut stream, "film2");
    send_tcp_packet(&mut stream, "insert entry 1");
    send_tcp_packet(&mut stream, "");

    // Add entry 1 to timescript
    send_tcp_packet(&mut stream, "script1");
    send_tcp_packet(&mut stream, "insert 1:00:17,04 entry 1");
    send_tcp_packet(&mut stream, "");

    // Exit
    send_tcp_packet(&mut stream, "exit");

    // Close the TCP/IP stream
    drop(stream);
}
