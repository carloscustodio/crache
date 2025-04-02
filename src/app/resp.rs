use std::io::{Cursor, Read};

pub struct Value {
    pub typ: String,       // equivalent to Go's `string`
    pub str: String,       // equivalent to Go's `string`
    pub num: i32,          // equivalent to Go's `int` (change to i64 if needed)
    pub bulk: String,      // equivalent to Go's `string`
    pub array: Vec<Value>, // equivalent to Go's `[]Value`
}

pub struct Resp {
    reader: Result<Cursor<Vec<u8>>, std::io::Error>,
}

pub fn check_input(input: &str) -> &'static str {
    let mut cursor = Cursor::new(input.as_bytes());

    // Read first byte and check it is '$'
    let mut first_byte = [0u8; 1];
    if cursor.read_exact(&mut first_byte).is_err() || first_byte[0] != b'$' {
        return "Error";
    }

    // Read size byte and check if it's a digit
    let mut size_byte = [0u8; 1];
    if cursor.read_exact(&mut size_byte).is_err() || !(b'0'..=b'9').contains(&size_byte[0]) {
        return "Error";
    }
    let size: usize = (size_byte[0] - b'0') as usize;
    println!("size: {}", size);

    // Check for "\r\n"
    let mut newline = [0u8; 2];
    if cursor.read_exact(&mut newline).is_err() || newline != *b"\r\n" {
        return "Error";
    }

    // Read exactly `size` bytes
    let mut name = vec![0u8; size];
    if cursor.read_exact(&mut name).is_err() {
        return "Error";
    }
    println!("{}", String::from_utf8_lossy(&name));

    "Success"
}

 
impl Resp {
    // Create a new Resp with empty data
    pub fn new_resp() -> Self {
        Resp {
            reader: Ok(Cursor::new(vec![])),
        }
    }

    // A constructor that accepts a vector of bytes
    pub fn new(reader_data: Vec<u8>) -> Self {
        Resp {
            reader: Ok(Cursor::new(reader_data)),
        }
    }

    

    pub fn read_line(&mut self) -> Result<(Vec<u8>, usize), std::io::Error> {
        let cursor = match self.reader.as_mut() {
            Ok(c) => c,
            Err(e) => return Err(std::io::Error::new(e.kind(), "previous error state")),
        };
        let mut line = Vec::new();
        let mut count = 0;
        loop {
            let mut buf = [0u8; 1];
            // Attempt to read one byte; return an error if reading fails.
            cursor.read_exact(&mut buf)?;
            count += 1;
            line.push(buf[0]);
            // When we have at least two bytes and the second to last is '\r', assume the line ended.
            if line.len() >= 2 && line[line.len() - 2] == b'\r' {
                break;
            }
        }
        // Ensure that there is a trailing "\r\n" to remove.
        if line.len() < 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "line too short",
            ));
        }
        // Remove the trailing "\r\n"
        line.truncate(line.len() - 2);
        Ok((line, count))
    }

    pub fn read(&mut self) -> Result<Value, std::io::Error> {
        // Get mutable access to the cursor.
        let cursor = match self.reader.as_mut() {
            Ok(c) => c,
            Err(e) => return Err(std::io::Error::new(e.kind(), "previous error state")),
        };
        // Read a single byte for the type
        let mut buf = [0u8; 1];
        cursor.read_exact(&mut buf)?;
        let type_byte = buf[0];

        // Match on the type byte
        match type_byte {
            b'*' => {
                let array = self.read_array()?;
                Ok(Value {
                    typ: "*".to_string(),
                    str: String::new(),
                    num: 0,
                    bulk: String::new(),
                    array,
                })
            },
           // b'$' => self.read_bulk(),
            _ => {
                println!("Unknown type: {}", type_byte as char);
                // Return an empty Value (or you can choose to return an error)
                Ok(Value {
                    typ: String::new(),
                    str: String::new(),
                    num: 0,
                    bulk: String::new(),
                    array: vec![],
                })
            }
        }
    }

    pub fn read_integer(&mut self) -> Result<i32, std::io::Error> {
        // Call our existing read_line method.
        let (line, n) = self.read_line()?;
        // Convert the line (Vec<u8>) into a &str.
        let s = std::str::from_utf8(&line)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
        // Trim any whitespace and attempt to parse the integer.
        let int_val = s.trim().parse::<i64>().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Cannot parse int")
        })?;
        Ok(int_val as i32)
    }
    pub fn read_array(&mut self) -> Result<Vec<Value>, std::io::Error> {
        
        let v = Resp::new_resp();
        let length: usize = self.read_integer()?.try_into().map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid size"))?;
       
        let mut array: Vec<Value> = Vec::with_capacity(length);
        for _ in 0..length {
            let val = self.read()?;
            array.push(val);
        }
       

        Ok(array)
    }

}