

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;



fn handle_client(mut stream: TcpStream) {
    // Buffer to store incoming data
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            // Echo the data back to the client
            let message = String::from_utf8_lossy(&buffer[0..size]);
            println!("Received: {}", message);
            
            if let Err(e) = stream.write_all(&buffer[0..size]) {
                println!("Failed to send response: {}", e);
            }
        }
        Err(e) => {
            println!("Error reading from connection: {}", e);
        }
    }
}

fn main() {
    // Create a TCP listener bound to address 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on port 8080");

    // Accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Spawn a new thread for each connection
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
