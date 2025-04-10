use std::{
    io::{Cursor, Read},
    vec,
};
#[derive(Clone)]
pub struct Value {
    pub typ: String,
    pub str: String,       // equivalent to Go's `string`
    pub num: i32,          // equivalent to Go's `int` (change to i64 if needed)
    pub bulk: String,      // equivalent to Go's `string` (changed from Vec<u8> to String)
    pub array: Vec<Value>, // equivalent to Go's `[]Value` (changed from Vec<u8> to Vec<Value>)
}

impl Value {
    pub fn marshal(&self) -> Vec<u8> {
        match self.typ.as_str() {
            "array" => self.array_marshal(),
            "bulk" => self.bulk_marshal(),
            "string" => self.string_marshal(),
            "integer" => self.integer_marshal(),
            "null" => self.null_marshal(),
            "error" => self.error_marshal(),
            _ => return vec![],
        }
    }

    pub fn null_marshal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.push(b'$'); // Use '$' for null type
        result.extend_from_slice(b"-1"); // Use '-' to indicate null value
        result.push(b'\r');
        result.push(b'\n');
        result
    }

    pub fn error_marshal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.push(b'-'); // Use '-' for error type
        result.extend_from_slice(self.str.as_bytes());
        result.push(b'\r');
        result.push(b'\n');
        result
    }

    pub fn string_marshal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.push(b'+'); // Use '+' for string type
        result.extend_from_slice(self.str.as_bytes());
        result.push(b'\r');
        result.push(b'\n');
        result
    }

    pub fn bulk_marshal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.push(b'$'); // Use '$' for bulk type
        result.extend_from_slice(self.bulk.len().to_string().as_bytes());
        result.push(b'\r');
        result.push(b'\n');
        result.extend_from_slice(self.bulk.as_bytes());
        result.push(b'\r');
        result.push(b'\n');
        result
    }

    pub fn integer_marshal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.push(b':'); // Use ':' for integer type
        result.extend_from_slice(self.num.to_string().as_bytes());
        result.push(b'\r');
        result.push(b'\n');
        result
    }

    pub fn array_marshal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.push(b'*'); // Use '*' for array type
        result.extend_from_slice(self.array.len().to_string().as_bytes());
        result.push(b'\r');
        result.push(b'\n');

        // Marshal each value in the array
        for value in &self.array {
            result.extend_from_slice(&value.marshal());
        }

        result
    }

    pub fn print(&self) -> String {
        match self.typ.as_str() {
            "array" => {
                let mut items = Vec::new();
                for val in &self.array {
                    items.push(val.print());
                }
                format!("Array: [{}]", items.join(", "))
            }
            "bulk" => format!("Bulk: \"{}\"", self.bulk),
            "string" => format!("String: \"{}\"", self.str),
            "integer" => format!("Integer: {}", self.num),
            _ => "Unknown".to_string(),
        }
    }
}

pub struct Resp {
    pub reader: Result<Cursor<Vec<u8>>, std::io::Error>,
}

impl Resp {
    // Create a new Resp with empty data
    pub fn new_resp() -> Self {
        Resp {
            reader: Ok(Cursor::new(vec![])),
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
            b'*' => self.read_array(),
            b'$' => self.read_bulk(),
            b'+' => {
                let (line, _) = self.read_line()?;
                let str_val = String::from_utf8(line).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")
                })?;
                Ok(Value {
                    typ: "string".to_string(),
                    str: str_val,
                    num: 0,
                    bulk: String::new(),
                    array: vec![],
                })
            }
            b':' => {
                let num_val = self.read_integer()?;
                Ok(Value {
                    typ: "integer".to_string(),
                    str: String::new(),
                    num: num_val,
                    bulk: String::new(),
                    array: vec![],
                })
            }
            _ => Ok(Value {
                typ: "error".to_string(),
                str: format!("Unexpected type byte: {}", type_byte as char),
                num: 0,
                bulk: String::new(),
                array: vec![],
            }),
        }
    }

    pub fn read_integer(&mut self) -> Result<i32, std::io::Error> {
        // Call our existing read_line method.
        let (line, _) = self.read_line()?;
        // Convert the line (Vec<u8>) into a &str.
        let s = std::str::from_utf8(&line)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
        // Trim any whitespace and attempt to parse the integer.
        let int_val = s.trim().parse::<i64>().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Cannot parse int")
        })?;
        Ok(int_val as i32)
    }

    pub fn read_array(&mut self) -> Result<Value, std::io::Error> {
        // Read the length of the array
        let length = self.read_integer()? as usize;

        // Create an array to store values
        let mut array = Vec::with_capacity(length);

        // Read each value recursively
        for _ in 0..length {
            let val = self.read()?;
            array.push(val);
        }

        Ok(Value {
            typ: "array".to_string(),
            str: String::new(),
            num: 0,
            bulk: String::new(),
            array:array,
        })
    }

    pub fn read_bulk(&mut self) -> Result<Value, std::io::Error> {
        // Read the length of the bulk string
        let length = self.read_integer()? as usize;

        // Read exactly length bytes for the content
        let mut buffer = vec![0u8; length];
        let cursor = match self.reader.as_mut() {
            Ok(c) => c,
            Err(e) => return Err(std::io::Error::new(e.kind(), "previous error state")),
        };

        cursor.read_exact(&mut buffer)?;

        // Read the trailing \r\n
        let mut crlf = [0u8; 2];
        cursor.read_exact(&mut crlf)?;
        if crlf != *b"\r\n" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing CRLF after bulk string",
            ));
        }

        // Convert binary data to UTF-8 string
        let bulk_str = String::from_utf8(buffer).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UTF-8 in bulk string",
            )
        })?;

        Ok(Value {
            typ: "bulk".to_string(),
            str: String::new(),
            num: 0,
            bulk: bulk_str,
            array: vec![],
        })
    }
}

pub struct Writer<W: std::io::Write> {
    pub writer: W,
}

impl<W: std::io::Write> Writer<W> {
    pub fn new(writer: W) -> Self {
        Writer { writer }
    }

    pub fn write(&mut self, v: &Value) -> Result<(), std::io::Error> {
        let bytes = v.marshal();
        self.writer.write_all(&bytes)?;
        Ok(())
    }
}
