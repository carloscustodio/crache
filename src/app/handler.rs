use std::collections::HashMap;
use crate::app::resp::Value;

type HandlerFunc = fn(Vec<Value>) -> Vec<u8>;

pub struct CommandHandler {
    handlers: HashMap<String, HandlerFunc>,
}

fn ping_handler(_args: Vec<Value>) -> Vec<u8> {
    let value = Value {
        typ: "string".to_string(),
        str: "PONG".to_string(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    };
    value.marshal()
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();
        handlers.insert("PING".to_string(), ping_handler as HandlerFunc);
        CommandHandler { handlers }
    }

    pub fn handle_command(&self, command: String, args: Vec<Value>) -> Vec<u8> {
        if let Some(handler) = self.handlers.get(&command) {
            handler(args)
        } else {
            let value = Value {
                typ: "error".to_string(),
                str: "ERR unknown command".to_string(),
                num: 0,
                bulk: String::new(),
                array: vec![],
            };
            value.marshal()
        }
    }
}
