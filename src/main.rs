use crache::app::resp::{Resp, Value, Writer};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread; // added import

fn handle_client(mut stream: TcpStream) {
    // Buffer to store incoming data
    let mut buffer = Vec::new();
    let mut command = String::new();
    let mut args = Vec::new();
    let mut handler = crache::app::handler::CommandHandler::new();

    if let Err(e) = stream.read_to_end(&mut buffer) {
        println!("Error reading stream: {}", e);
        return;
    }
    println!("Buffer: {:?}", buffer);
    let mut resp = Resp {
        reader: Ok(std::io::Cursor::new(buffer)),
    };
    let mut writer = Writer {
        writer: std::io::Cursor::new(Vec::new()),
    };
    match resp.read() {
        Ok(val) =>{
            command = val.array[0].str.clone().to_uppercase();
            args = val.array[1..].to_vec();
},
        Err(e) => println!("Error parsing RESP: {}", e),
    }
    match writer.write(&Value{typ: "string".to_owned(), str: "OK".to_owned() , num: 0, bulk: "".to_owned(), array: vec![]}) {
        Ok(_) => println!("Successfully wrote RESP"),
        Err(e) => println!("Error writing RESP: {}", e),
    }

}

fn main() {
    // Create a TCP listener bound to address 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to address");
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
