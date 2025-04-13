use crache::app::handler::get_handler;
use crache::app::resp::{Resp, Writer};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let mut handler = None;
    let mut args = vec![];
    let mut command = String::new();

    // Process client request
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                let mut resp = Resp {
                    reader: Ok(std::io::Cursor::new(buffer[..n].to_vec())),
                };
                // Use the stream directly for writing back to the client.
                let mut writer = Writer {
                    writer: &mut stream,
                };
                match resp.read() {
                    Ok(val) => {
                        command = val.array[0].bulk.clone();
                        println!("val: {:?}", command);
                        args = val.array[1..].to_vec();
                        handler = get_handler(&command.to_string().to_ascii_uppercase());
                    }
                    Err(e) => println!("Error parsing RESP: {}", e),
                }
                if let Some(func) = handler {
                    let value = func(args);
                    if command == "SET" || command == "HSET" {
                        // Write the response to the AOF file
                        let aof = crache::app::aof::Aof::new("aof_file.aof");
                        aof.write(&value.marshal())
                            .expect("Failed to write to AOF file");
                    }
                    if let Err(e) = writer.write(&value) {
                        eprintln!("Error writing response: {}", e);
                    }
                } else {
                    eprintln!("No matching handler found.");
                }
            }
        }
        Err(e) => {
            println!("Error reading stream: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to address");
    println!("Server listening on port 6379");

    let aof = crache::app::aof::Aof::new("aof_file.aof");

    // Load and process all commands from AOF file when a client connects
    if let Err(e) = aof.read(|value| {
        if value.typ == "array" && !value.array.is_empty() {
            let command = value.array[0].bulk.to_ascii_uppercase();
            let args = value.array[1..].to_vec();

            if let Some(handler) = get_handler(&command) {
                println!("Replaying command from AOF: {}", command);
                handler(args);
            } else {
                println!("Invalid command in AOF: {}", command);
            }
        }
    }) {
        eprintln!("Error loading AOF file: {}", e);
    }


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
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
