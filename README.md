# Crache

Crache is a simple Rust project that demonstrates basic networking and RESP protocol parsing. It includes a TCP server that echoes incoming messages and utilities to parse and validate RESP-formatted data.

## Project Structure

- **[Cargo.toml](Cargo.toml):** Project configuration and dependencies.
- **src/**
  - **[main.rs](src/main.rs):** Entry point of the TCP server. It listens on port 8080 and spawns a new thread for each incoming connection.
  - **lib.rs:** Exposes project modules.
  - **app/resp.rs:** Contains functions to check and parse RESP protocol inputs (e.g., [`check_input`](src/app/resp.rs) and [`Resp`](src/app/resp.rs)).
  - **app/handler.rs & app/aof.rs:** (Reserved for future extensions such as custom command handling and append-only file logic.)
- **tests/resp_tests.rs:** Unit tests for validating RESP parsing functionality.

## Features

- **TCP Server:** 
  - Listens on `127.0.0.1:8080`.
  - Echoes received messages back to the client.
  
- **RESP Parsing:**
  - Validates input to ensure it conforms to RESP standards (e.g., starts with `$`).
  - Provides methods in [`Resp`](src/app/resp.rs) to read lines and integers from input data.

## Getting Started

1. **Build the project:**

   ```sh
   cargo build
