use crache::app::handler::get_handler;
use crache::app::resp::{Resp, Writer};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let mut handler = None;
    let mut args = vec![];
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n > 0 {
                let mut resp = Resp {
                    reader: Ok(std::io::Cursor::new(buffer[..n].to_vec()))
                };
                // Use the stream directly for writing back to the client.
                let mut writer = Writer {
                    writer: &mut stream
                };
                match resp.read() {
                    Ok(val) => {
                        let command = &val.array[0].bulk; 
                        println!("val: {:?}", command);               
                        args = val.array[1..].to_vec();
                        handler = get_handler(&command.to_string());
                    }
                    Err(e) => println!("Error parsing RESP: {}", e),
                }
                if let Some(func) = handler {
                    let value = func(args);
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
