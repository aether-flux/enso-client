use std::{io::{BufRead, BufReader, Write}, net::TcpStream};

use crate::{error::{EnsoError}, protocol::EOF_MARKER};

pub struct Enso {
    reader: BufReader<TcpStream>,
}

impl Enso {
    pub fn connect(addr: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let reader = BufReader::new(stream);
        Ok(Self { reader })
    }

    pub fn execute(&mut self, query: &str) -> Result<String, EnsoError> {
        self.reader.get_mut().write_all(query.as_bytes())?;
        self.reader.get_mut().write_all(b"\n")?;

        let mut response = String::new();
        // self.reader.read_line(&mut response)?;

        loop {
            let mut line = String::new();
            self.reader.read_line(&mut line)?;

            if line.trim() == EOF_MARKER {
                break;
            }

            response.push_str(&line);
        }

        Ok(response)
    }
}
