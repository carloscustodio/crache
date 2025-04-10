use crate::app::resp::Value;

type HandlerFunc = fn(Vec<Value>) -> Value;

fn ping_handler(_args: Vec<Value>) -> Value {
    Value {
        typ: "string".to_string(),
        str: "PONG".to_string(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    }
}

pub fn get_handler(command: &str) -> Option<HandlerFunc> {
    match command {
        "PING" => Some(ping_handler),
        _ => None,
    }
}
