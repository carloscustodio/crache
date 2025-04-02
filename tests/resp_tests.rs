use crache::app::resp;

#[test]
fn test_check_input_success() {
    // Test input starting with '$' character
    let result = resp::check_input("$3\r\nSET\r\n");
    assert_eq!(result, "Success");
}

#[test]
fn test_check_input_failure() {
    // Test input not starting with '$' character
    let result = resp::check_input("SET key value");
    assert_eq!(result, "Error");
}

#[test]
fn test_check_input_empty() {
    // Test with empty input
    let result = resp::check_input("");
    assert_eq!(result, "Error");
}

#[test]
fn test_check_input_special_chars() {
    // Test with other special characters
    let result = resp::check_input("*3\r\n$3\r\nSET\r\n");
    assert_eq!(result, "Error"); // Should fail as it starts with '*' not '$'
}

#[test]
fn test_read_line_success() {
    // Test read_line returns "Hello" with correct byte count
    let input = b"Hello\r\n".to_vec();
    let mut instance = resp::Resp::new(input);
    let (line, count) = instance.read_line().unwrap();
    assert_eq!(line, b"Hello".to_vec());
    assert_eq!(count, 7); // "Hello\r\n" is 7 bytes
}

#[test]
fn test_read_integer_success() {
    // Test read_integer returns 123 with correct byte count
    let input = b"123\r\n".to_vec();
    let mut instance = resp::Resp::new(input);
    let (num, count) = instance.read_integer().unwrap();
    assert_eq!(num, 123);
    assert_eq!(count, 5); // "123\r\n" is 5 bytes
}
