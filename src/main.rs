use crache::app::resp::Resp;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread; // added import

fn handle_client(mut stream: TcpStream) {
    // Buffer to store incoming data
    let mut buffer = Vec::new();
    if let Err(e) = stream.read_to_end(&mut buffer) {
        println!("Error reading stream: {}", e);
        return;
    }
    let mut resp = Resp {
        reader: Ok(std::io::Cursor::new(buffer)),
    };
    match resp.read() {
        Ok(val) => println!("Received value of type: {}", val.typ),
        Err(e) => println!("Error parsing RESP: {}", e),
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
