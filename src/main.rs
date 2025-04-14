use crache::app::handler::get_handler;
use crache::app::resp::{Resp, Writer, Value};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    
    loop {
        // Process client request
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Client disconnected");
                    break;
                }
                
                let mut resp = Resp {
                    reader: Ok(std::io::Cursor::new(buffer[..n].to_vec())),
                };
                let mut writer = Writer {
                    writer: &mut stream,
                };

                match resp.read() {
                    Ok(val) => {
                        if val.array.is_empty() {
                            eprintln!("Error: Empty command array received");
                            // Consider sending a RESP error back to the client here
                            continue;
                        }

                        let command = val.array[0].bulk.clone().to_ascii_uppercase();
                        println!("Received command: {:?}", command);
                        
                        let handler = get_handler(&command.to_string().to_ascii_uppercase());

                        if let Some(func) = handler {
                            let args = val.array[1..].to_vec(); // Extract args only if handler exists
                            let value = func(args.clone()); // Clone args for potential AOF use
                            
                            // AOF Check
                            if command == "SET" || command == "HSET" {
                                let aof = crache::app::aof::Aof::new("aof_file.aof");
                                let mut command_args_values = vec![Value {
                                    typ: "bulk".to_string(),
                                    str: String::new(),
                                    num: 0,
                                    bulk: command.clone(), // Use the command captured earlier
                                    array: vec![],
                                }];
                                command_args_values.extend(args.iter().map(|arg| Value {
                                    typ: "bulk".to_string(),
                                    str: String::new(),
                                    num: 0,
                                    bulk: arg.bulk.clone(),
                                    array: vec![],
                                }));
                                let resp_value = Value {
                                    typ: "array".to_string(),
                                    str: String::new(),
                                    num: 0,
                                    bulk: String::new(),
                                    array: command_args_values,
                                };
                                aof.write(&resp_value.marshal())
                                    .expect("Failed to write to AOF file");
                            }

                            // Write response to client
                            if let Err(e) = writer.write(&value) {
                                eprintln!("Error writing response: {}", e);
                                break;
                            }
                        } else {
                            // No handler found for the command
                            eprintln!("No matching handler found for command: {}", command);
                            // Send an error response back to the client
                            let error_response = Value {
                                typ: "error".to_string(),
                                str: format!("ERR unknown command '{}'", command),
                                num: 0,
                                bulk: String::new(),
                                array: vec![],
                            };
                            if let Err(e) = writer.write(&error_response) {
                                eprintln!("Error writing error response: {}", e);
                                break; // Exit loop on write error
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error parsing RESP: {}", e);
                        // Optionally send a RESP error back to the client
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("Error reading stream: {}", e);
                break;
            }
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
