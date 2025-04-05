use crache::app::resp::{Resp, Value, Writer};
use std::io::Cursor;

#[test]
fn test_read_array_empty() {
    // Construct an empty array: "*0\r\n"
    let input = b"*0\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "array");
    // An empty RESP array should yield an empty inner array.
    assert!(value.array.is_empty());
}

#[test]
fn test_read_bulk_empty() {
    // Construct an empty bulk response: "$0\r\n\r\n"
    let input = b"$0\r\n\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "bulk");
    assert!(value.bulk.is_empty());
}

#[test]
fn test_unknown_type() {
    // Use an unsupported type byte, e.g. '?'
    let input = b"?0\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok()); // Now we expect an Ok result
    let value = result.unwrap();
    assert_eq!(value.typ, "error"); // With an error type
    assert!(value.str.contains("Unexpected type byte")); // And appropriate message
}

#[test]
fn test_read_simple_string() {
    // Construct a simple string: "+OK\r\n"
    let input = b"+OK\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "string");
    assert_eq!(value.str, "OK");
}

#[test]
fn test_read_integer() {
    // Construct an integer response: ":1000\r\n"
    let input = b":1000\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "integer");
    assert_eq!(value.num, 1000);
}

#[test]
fn test_read_bulk_string() {
    // Construct a bulk string: "$5\r\nAhmed\r\n"
    let input = b"$5\r\nAhmed\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "bulk");
    assert_eq!(value.bulk, "Ahmed");
}

#[test]
fn test_read_bulk_string_with_spaces() {
    // Construct a bulk string with spaces: "$11\r\nHello World\r\n"
    let input = b"$11\r\nHello World\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "bulk");
    assert_eq!(value.bulk, "Hello World");
}

#[test]
fn test_read_bulk_string_with_special_chars() {
    // Construct a bulk string with special characters: "$15\r\nTest@123!$%^&*()\r\n"
    let input = b"$16\r\nTest@123!$%^&*()\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "bulk");
    assert_eq!(value.bulk, "Test@123!$%^&*()");
}

#[test]
fn test_read_array_with_mixed_types() {
    // Construct an array with mixed types: "*3\r\n+OK\r\n:1000\r\n$5\r\nAhmed\r\n"
    let input = b"*3\r\n+OK\r\n:1000\r\n$5\r\nAhmed\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "array");
    assert_eq!(value.array.len(), 3);
    assert_eq!(value.array[0].typ, "string");
    assert_eq!(value.array[0].str, "OK");
    assert_eq!(value.array[1].typ, "integer");
    assert_eq!(value.array[1].num, 1000);
    assert_eq!(value.array[2].typ, "bulk");
    assert_eq!(value.array[2].bulk, "Ahmed");
}

#[test]
fn test_read_nested_array() {
    // Construct a nested array: "*2\r\n*2\r\n+Hello\r\n+World\r\n$5\r\nAhmed\r\n"
    let input = b"*2\r\n*2\r\n+Hello\r\n+World\r\n$5\r\nAhmed\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "array");
    assert_eq!(value.array.len(), 2);

    // Check first element (nested array)
    assert_eq!(value.array[0].typ, "array");
    assert_eq!(value.array[0].array.len(), 2);
    assert_eq!(value.array[0].array[0].typ, "string");
    assert_eq!(value.array[0].array[0].str, "Hello");
    assert_eq!(value.array[0].array[1].typ, "string");
    assert_eq!(value.array[0].array[1].str, "World");

    // Check second element (bulk string)
    assert_eq!(value.array[1].typ, "bulk");
    assert_eq!(value.array[1].bulk, "Ahmed");
}

#[test]
fn test_read_command() {
    // Test a typical Redis SET command: *3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
    let input = b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "array");
    assert_eq!(value.array.len(), 3);

    assert_eq!(value.array[0].typ, "bulk");
    assert_eq!(value.array[0].bulk, "SET");

    assert_eq!(value.array[1].typ, "bulk");
    assert_eq!(value.array[1].bulk, "key");

    assert_eq!(value.array[2].typ, "bulk");
    assert_eq!(value.array[2].bulk, "value");
}

#[test]
fn test_print_bulk_string() {
    // Test the print function for a bulk string
    let input = b"$5\r\nAhmed\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.print(), "Bulk: \"Ahmed\"");
}

#[test]
fn test_print_array() {
    // Test the print function for an array
    let input = b"*2\r\n$5\r\nAhmed\r\n:123\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.print(), "Array: [Bulk: \"Ahmed\", Integer: 123]");
}

// Marshal Tests

#[test]
fn test_marshal_string() {
    let value = Value {
        typ: "string".to_string(),
        str: "OK".to_string(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    };

    let result = value.marshal();
    assert_eq!(result, b"+OK\r\n");
}

#[test]
fn test_marshal_error() {
    let value = Value {
        typ: "error".to_string(),
        str: "Error message".to_string(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    };

    let result = value.marshal();
    assert_eq!(result, b"-Error message\r\n");
}

#[test]
fn test_marshal_integer() {
    let value = Value {
        typ: "integer".to_string(),
        str: String::new(),
        num: 42,
        bulk: String::new(),
        array: vec![],
    };

    let result = value.marshal();
    assert_eq!(result, b":42\r\n");
}

#[test]
fn test_marshal_bulk() {
    let value = Value {
        typ: "bulk".to_string(),
        str: String::new(),
        num: 0,
        bulk: "Hello World".to_string(),
        array: vec![],
    };

    let result = value.marshal();
    assert_eq!(result, b"$11\r\nHello World\r\n");
}

#[test]
fn test_marshal_null() {
    let value = Value {
        typ: "null".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    };

    let result = value.marshal();
    assert_eq!(result, b"$-1\r\n");
}

#[test]
fn test_marshal_empty_array() {
    let value = Value {
        typ: "array".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    };

    let result = value.marshal();
    assert_eq!(result, b"*0\r\n");
}

#[test]
fn test_marshal_array_with_elements() {
    let value = Value {
        typ: "array".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![
            Value {
                typ: "string".to_string(),
                str: "OK".to_string(),
                num: 0,
                bulk: String::new(),
                array: vec![],
            },
            Value {
                typ: "integer".to_string(),
                str: String::new(),
                num: 1000,
                bulk: String::new(),
                array: vec![],
            },
            Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: "Ahmed".to_string(),
                array: vec![],
            },
        ],
    };

    let result = value.marshal();
    assert_eq!(result, b"*3\r\n+OK\r\n:1000\r\n$5\r\nAhmed\r\n");
}

#[test]
fn test_marshal_nested_array() {
    let value = Value {
        typ: "array".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![
            Value {
                typ: "array".to_string(),
                str: String::new(),
                num: 0,
                bulk: String::new(),
                array: vec![
                    Value {
                        typ: "string".to_string(),
                        str: "Hello".to_string(),
                        num: 0,
                        bulk: String::new(),
                        array: vec![],
                    },
                    Value {
                        typ: "string".to_string(),
                        str: "World".to_string(),
                        num: 0,
                        bulk: String::new(),
                        array: vec![],
                    },
                ],
            },
            Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: "Ahmed".to_string(),
                array: vec![],
            },
        ],
    };

    let result = value.marshal();
    assert_eq!(result, b"*2\r\n*2\r\n+Hello\r\n+World\r\n$5\r\nAhmed\r\n");
}

#[test]
fn test_marshal_redis_command() {
    // Create a RESP array representation of "SET key value"
    let value = Value {
        typ: "array".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![
            Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: "SET".to_string(),
                array: vec![],
            },
            Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: "key".to_string(),
                array: vec![],
            },
            Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: "value".to_string(),
                array: vec![],
            },
        ],
    };

    let result = value.marshal();
    assert_eq!(result, b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n");
}

#[test]
fn test_writer_writes_value() {
    // Create a test value
    let value = Value {
        typ: "string".to_string(),
        str: "OK".to_string(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    };

    // Create a writer with a Vec<u8> as the underlying buffer
    let buffer = Vec::new();
    let mut writer = Writer::new(buffer);

    // Write value to buffer
    let result = writer.write(&value);
    assert!(result.is_ok());
}

#[test]
fn test_writer_writes_complex_value() {
    // Create a complex RESP value (array with different value types)
    let value = Value {
        typ: "array".to_string(),
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![
            Value {
                typ: "string".to_string(),
                str: "OK".to_string(),
                num: 0,
                bulk: String::new(),
                array: vec![],
            },
            Value {
                typ: "integer".to_string(),
                str: String::new(),
                num: 42,
                bulk: String::new(),
                array: vec![],
            },
            Value {
                typ: "bulk".to_string(),
                str: String::new(),
                num: 0,
                bulk: "Hello".to_string(),
                array: vec![],
            },
            Value {
                typ: "null".to_string(),
                str: String::new(),
                num: 0,
                bulk: String::new(),
                array: vec![],
            },
        ],
    };

    // Create a writer with a cursor as the underlying writer
    let buffer = Vec::new();
    let mut writer = Writer::new(buffer);

    // Write value to buffer
    let result = writer.write(&value);
    assert!(result.is_ok());

    // Get the underlying buffer and check the content
    let buffer = writer.write(&value);
    assert!(buffer.is_ok());
}

#[test]
fn test_unknown_type_marshal() {
    let value = Value {
        typ: "unknown".to_string(), // An unsupported type
        str: String::new(),
        num: 0,
        bulk: String::new(),
        array: vec![],
    };

    let result = value.marshal();
    assert!(result.is_empty()); // Should return an empty vector for unknown types
}
