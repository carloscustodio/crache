use crate::app::resp::Value;
use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

type HandlerFunc = fn(Vec<Value>) -> Value;

lazy_static! {
    static ref SETS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
    static ref HSETS: RwLock<HashMap<String, HashMap<String, String>>> =
        RwLock::new(HashMap::new());
}

fn ping_handler(_args: Vec<Value>) -> Value {
    if _args.len() == 0 {
        Value {
            typ: "string".to_string(),
            str: "PONG".to_string(),
            num: 0,
            bulk: String::new(),
            array: vec![],
        }
    } else {
        Value {
            typ: "string".to_string(),
            str: _args[0].bulk.to_string(),
            num: 0,
            bulk: String::new(),
            array: vec![],
        }
    }
}

fn fn_get_handler(_args: Vec<Value>) -> Value {
    let map = SETS.read().unwrap();
    if let Some(val) = map.get(_args[0].bulk.as_str()) {
        // If found, return the associated value.
        Value {
            typ: "bulk".to_string(),
            str: "".to_string(),
            num: 0,
            bulk: val.clone(),
            array: vec![],
        }
    } else {
        // If not found, return a null Value.
        Value {
            typ: "null".to_string(),
            str: "".to_string(),
            num: 0,
            bulk: String::new(),
            array: vec![],
        }
    }
}

fn set_handler(_args: Vec<Value>) -> Value {
    let mut map = SETS.write().unwrap();
    map.insert(_args[0].bulk.clone(), _args[1].bulk.clone());
    Value {
        typ: "string".to_string(),
        str: "OK".to_string(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    }
}

fn hset_handler(_args: Vec<Value>) -> Value {
    if _args.len() != 3 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments for 'hset' command".to_string(),
            num: 0,
            bulk: String::new(),
            array: vec![],
        };
    }

    let hash = _args[0].bulk.clone();
    let key = _args[1].bulk.clone();
    let value = _args[2].bulk.clone();

    let mut hsets = HSETS.write().unwrap();
    if !hsets.contains_key(&hash) {
        hsets.insert(hash.clone(), HashMap::new());
    }

    if let Some(hash_map) = hsets.get_mut(&hash) {
        hash_map.insert(key, value);
    }

    Value {
        typ: "string".to_string(),
        str: "OK".to_string(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    }
}

fn hget_handler(_args: Vec<Value>) -> Value {
    if _args.len() != 2 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments for 'hget' command".to_string(),
            num: 0,
            bulk: String::new(),
            array: vec![],
        };
    }

    let hash = _args[0].bulk.clone();
    let key = _args[1].bulk.clone();

    let hsets = HSETS.read().unwrap();

    if let Some(hash_map) = hsets.get(&hash) {
        if let Some(value) = hash_map.get(&key) {
            return Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: value.clone(),
                array: vec![],
            };
        }
    }

    // If key not found, return null
    Value {
        typ: "null".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    }
}

fn hgetall_handler(_args: Vec<Value>) -> Value {
    if _args.len() != 1 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments for 'hgetall' command".to_string(),
            num: 0,
            bulk: String::new(),
            array: vec![],
        };
    }

    let hash = _args[0].bulk.clone();
    let hsets = HSETS.read().unwrap();

    if let Some(hash_map) = hsets.get(&hash) {
        let mut result = Vec::new();

        // For each key-value pair in the hash, add both key and value to the result array
        for (field, value) in hash_map {
            // Add field
            result.push(Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: field.clone(),
                array: vec![],
            });

            // Add value
            result.push(Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: value.clone(),
                array: vec![],
            });
        }

        return Value {
            typ: "array".to_string(),
            str: String::new(),
            num: 0,
            bulk: String::new(),
            array: result,
        };
    }

    // If the hash does not exist, return an empty array
    Value {
        typ: "array".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    }
}

pub fn get_handler(command: &str) -> Option<HandlerFunc> {
    match command {
        "PING" => Some(ping_handler),
        "SET" => Some(set_handler),
        "GET" => Some(fn_get_handler),
        "HSET" => Some(hset_handler),
        "HGET" => Some(hget_handler),
        "HGETALL" => Some(hgetall_handler),
        _ => None,
    }
}
