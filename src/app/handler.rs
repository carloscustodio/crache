use std::collections::HashMap;
use crate::app::resp::Value;

type HandlerFunc = fn(Vec<Value>) -> Value;

pub struct CommandHandler {
    handlers: HashMap<String, HandlerFunc>,
}

fn ping_handler(_args: Vec<Value>) -> Value {
    Value::string_marshal("PONG")
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();
        handlers.insert("PING".to_string(), ping_handler);
        CommandHandler { handlers }
    }

    pub fn handle_command(&self, command: String, args: Vec<Value>) -> Value {
        if let Some(handler) = self.handlers.get(&command) {
            handler(args)
        } else {
            Value::error_marshal("ERR unknown command")
        }
    }
}