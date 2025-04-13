use crache::app::handler;
use crache::app::resp::Value;

// Helper function to create bulk string values for testing
fn bulk_string(s: &str) -> Value {
    Value {
        typ: "bulk".to_string(),
        str: String::new(),
        num: 0,
        bulk: s.to_string(),
        array: vec![],
    }
}

#[test]
fn test_ping_handler_no_args() {
    let args = vec![];
    if let Some(handler_fn) = handler::get_handler("PING") {
        let result = handler_fn(args);
        assert_eq!(result.typ, "string");
        assert_eq!(result.str, "PONG");
    } else {
        panic!("PING handler not found");
    }
}

#[test]
fn test_ping_handler_with_arg() {
    let args = vec![bulk_string("hello")];
    if let Some(handler_fn) = handler::get_handler("PING") {
        let result = handler_fn(args);
        assert_eq!(result.typ, "string");
        assert_eq!(result.str, "hello");
    } else {
        panic!("PING handler not found");
    }
}

#[test]
fn test_set_and_get_handler() {
    // Test SET
    let key = "test_key";
    let value = "test_value";
    let set_args = vec![bulk_string(key), bulk_string(value)];

    if let Some(set_fn) = handler::get_handler("SET") {
        let set_result = set_fn(set_args);
        assert_eq!(set_result.typ, "string");
        assert_eq!(set_result.str, "OK");

        // Test GET
        let get_args = vec![bulk_string(key)];
        if let Some(get_fn) = handler::get_handler("GET") {
            let get_result = get_fn(get_args);
            assert_eq!(get_result.typ, "bulk");
            assert_eq!(get_result.bulk, value);
        } else {
            panic!("GET handler not found");
        }
    } else {
        panic!("SET handler not found");
    }
}

#[test]
fn test_get_nonexistent_key() {
    let args = vec![bulk_string("nonexistent_key")];
    if let Some(handler_fn) = handler::get_handler("GET") {
        let result = handler_fn(args);
        assert_eq!(result.typ, "null");
    } else {
        panic!("GET handler not found");
    }
}

#[test]
fn test_hset_handler_invalid_args() {
    // Test with too few arguments
    let invalid_args = vec![bulk_string("hash"), bulk_string("field")]; // Missing value

    if let Some(handler_fn) = handler::get_handler("HSET") {
        let result = handler_fn(invalid_args);
        assert_eq!(result.typ, "error");
        assert!(result.str.contains("wrong number of arguments"));
    } else {
        panic!("HSET handler not found");
    }
}

#[test]
fn test_hget_handler_invalid_args() {
    // Test with too few arguments
    let invalid_args = vec![bulk_string("hash")]; // Missing field

    if let Some(handler_fn) = handler::get_handler("HGET") {
        let result = handler_fn(invalid_args);
        assert_eq!(result.typ, "error");
        assert!(result.str.contains("wrong number of arguments"));
    } else {
        panic!("HGET handler not found");
    }
}

#[test]
fn test_hset_and_hget_handler() {
    // Test HSET
    let hash = "test_hash";
    let field = "test_field";
    let value = "test_value";
    let hset_args = vec![bulk_string(hash), bulk_string(field), bulk_string(value)];

    if let Some(hset_fn) = handler::get_handler("HSET") {
        let hset_result = hset_fn(hset_args);
        assert_eq!(hset_result.typ, "string");
        assert_eq!(hset_result.str, "OK");

        // Test HGET
        let hget_args = vec![bulk_string(hash), bulk_string(field)];
        if let Some(hget_fn) = handler::get_handler("HGET") {
            let hget_result = hget_fn(hget_args);
            assert_eq!(hget_result.typ, "bulk");
            assert_eq!(hget_result.bulk, value);
        } else {
            panic!("HGET handler not found");
        }
    } else {
        panic!("HSET handler not found");
    }
}

#[test]
fn test_hget_nonexistent_key() {
    // Test HGET with nonexistent hash
    let hash = "nonexistent_hash";
    let field = "some_field";
    let hget_args = vec![bulk_string(hash), bulk_string(field)];

    if let Some(hget_fn) = handler::get_handler("HGET") {
        let result = hget_fn(hget_args);
        assert_eq!(result.typ, "null");
    } else {
        panic!("HGET handler not found");
    }
}

#[test]
fn test_hget_nonexistent_field() {
    // First set up a hash
    let hash = "test_hash2";
    let field = "existing_field";
    let value = "some_value";
    let hset_args = vec![bulk_string(hash), bulk_string(field), bulk_string(value)];

    if let Some(hset_fn) = handler::get_handler("HSET") {
        let hset_result = hset_fn(hset_args);
        assert_eq!(hset_result.typ, "string");

        // Now try to get a field that doesn't exist
        let nonexistent_field = "nonexistent_field";
        let hget_args = vec![bulk_string(hash), bulk_string(nonexistent_field)];

        if let Some(hget_fn) = handler::get_handler("HGET") {
            let result = hget_fn(hget_args);
            assert_eq!(result.typ, "null");
        } else {
            panic!("HGET handler not found");
        }
    } else {
        panic!("HSET handler not found");
    }
}

#[test]
fn test_hgetall_handler_invalid_args() {
    // Test with too many arguments
    let invalid_args = vec![bulk_string("hash"), bulk_string("extra")];

    if let Some(handler_fn) = handler::get_handler("HGETALL") {
        let result = handler_fn(invalid_args);
        assert_eq!(result.typ, "error");
        assert!(result.str.contains("wrong number of arguments"));
    } else {
        panic!("HGETALL handler not found");
    }
}

#[test]
fn test_hgetall_empty_hash() {
    // Test HGETALL on a hash that doesn't exist
    let hash = "nonexistent_hash";
    let hgetall_args = vec![bulk_string(hash)];

    if let Some(hgetall_fn) = handler::get_handler("HGETALL") {
        let result = hgetall_fn(hgetall_args);
        assert_eq!(result.typ, "array");
        assert!(result.array.is_empty());
    } else {
        panic!("HGETALL handler not found");
    }
}

#[test]
fn test_hgetall_handler() {
    // Set up a hash with multiple fields
    let hash = "test_hash3";
    let fields = [("name", "John"), ("age", "30"), ("city", "New York")];

    if let Some(hset_fn) = handler::get_handler("HSET") {
        // First set all the fields
        for (field, value) in &fields {
            let hset_args = vec![bulk_string(hash), bulk_string(field), bulk_string(value)];
            hset_fn(hset_args);
        }

        // Now test HGETALL
        let hgetall_args = vec![bulk_string(hash)];

        if let Some(hgetall_fn) = handler::get_handler("HGETALL") {
            let result = hgetall_fn(hgetall_args);

            // Verify the result
            assert_eq!(result.typ, "array");
            assert_eq!(result.array.len(), 6); // 3 fields * 2 (field + value)

            // We need to check all field-value pairs are present
            // Since hash iteration order is not guaranteed, we need to collect all fields and values
            let mut found_fields = Vec::new();
            let mut found_values = Vec::new();

            for i in (0..result.array.len()).step_by(2) {
                found_fields.push(result.array[i].bulk.clone());
                found_values.push(result.array[i + 1].bulk.clone());
            }

            // Check each expected field-value pair exists
            for (field, value) in &fields {
                let field_pos = found_fields.iter().position(|f| f == field);
                assert!(field_pos.is_some());

                let index = field_pos.unwrap();
                assert_eq!(found_values[index], *value);
            }
        } else {
            panic!("HGETALL handler not found");
        }
    } else {
        panic!("HSET handler not found");
    }
}

#[test]
fn test_get_handler_unknown_command() {
    let handler = handler::get_handler("UNKNOWN_COMMAND");
    assert!(handler.is_none());
}
