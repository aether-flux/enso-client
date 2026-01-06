# Enso Client

**enso-client** is the official Rust client library for connecting to an EnsoDB server over TCP.

It provides a simple, synchronous API for executing queries and receiving results.

> ⚠️ This crate is experimental and targets EnsoDB v0.1.

---

## Features

- TCP-based connection to EnsoDB
- Simple `execute()` API
- Text-based protocol
- No CLI — library only

---

## Installation
```bash
cargo add enso-client
```

---

## Usage
```rust
use enso_client::Enso;
use enso_client::EnsoError;

fn main() -> Result<(), EnsoError> {
    let mut db = Enso::connect("127.0.0.1:5432")?;

    let res = db.execute("SELECT * FROM users;")?;
    println!("{}", res);

    Ok(())
}
```

---

## Design Philosophy
- Minimal API surface
- Explicit behavior
- Matches server protocol

This crate is intended for:
- embedding EnsoDB in Rust apps
- building higher-level abstractions
- learning how DB client libraries work

---

## Protocol Notes
- One query per line
- Responses may span multiple lines
- Responses terminate with a special EOF marker

This protocol is intentionally simple and unstable for now.

---

## License
MIT

