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
        Ok(n) if n > 0 => {
            let mut resp = Resp {
                reader: Ok(std::io::Cursor::new(buffer.to_vec()))
            };
            let mut writer = Writer {
                writer: std::io::Cursor::new(buffer)};
            match resp.read() {
                Ok(val) => {
                    let command = val.array[0].str.clone().to_uppercase();
                     args = val.array[1..].to_vec();
                     handler = get_handler(&command);

                }
                Err(e) => println!("Error parsing RESP: {}", e),
            }
            let value = handler.unwrap()(args);
             let _ = writer.write(&value);
        }
        Ok(_) => println!("Client disconnected"),
        Err(e) => println!("Error reading stream: {}", e),
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
