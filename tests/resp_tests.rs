use crache::app::resp::Resp;
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
    // For bulk, our implementation returns a Vec<Value> which should be empty.
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
    assert!(result.is_err());
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
    assert_eq!(value.str, "Ahmed");
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
    assert_eq!(value.str, "Hello World");
}

#[test]
fn test_read_bulk_string_with_special_chars() {
    // Construct a bulk string with special characters: "$15\r\nTest@123!$%^&*()\r\n"
    let input = b"$15\r\nTest@123!$%^&*()\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "bulk");
    assert_eq!(value.str, "Test@123!$%^&*()");
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
    assert_eq!(value.array[2].str, "Ahmed");
}

#[test]
fn test_read_null_bulk_string() {
    // Construct a null bulk string: "$-1\r\n"
    let input = b"$-1\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "bulk");
    assert!(value.str.is_empty());
}

#[test]
fn test_read_null_array() {
    // Construct a null array: "*-1\r\n"
    let input = b"*-1\r\n".to_vec();
    let mut resp = Resp {
        reader: Ok(Cursor::new(input)),
    };

    let result = resp.read();
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.typ, "array");
    assert!(value.array.is_empty());
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
    assert_eq!(value.array[1].str, "Ahmed");
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
    assert_eq!(value.array[0].str, "SET");

    assert_eq!(value.array[1].typ, "bulk");
    assert_eq!(value.array[1].str, "key");

    assert_eq!(value.array[2].typ, "bulk");
    assert_eq!(value.array[2].str, "value");
}
