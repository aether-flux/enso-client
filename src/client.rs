use std::{io::{BufRead, BufReader, BufWriter, Write}, net::TcpStream};

use crate::{error::{EnsoError}, protocol::EOF_MARKER};

pub struct Enso {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Enso {
    pub fn connect(addr: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nodelay(true)?;

        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn execute(&mut self, query: &str) -> Result<String, EnsoError> {
        self.writer.write_all(query.as_bytes())?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;

        // self.reader.get_mut().write_all(query.as_bytes())?;
        // self.reader.get_mut().write_all(b"\n")?;

        let mut response = String::new();
        // self.reader.read_line(&mut response)?;

        loop {
            let mut line = String::new();
            let bytes = self.reader.read_line(&mut line).unwrap_or(0);

            if bytes == 0 {
                break;
            }

            if line.trim() == EOF_MARKER {
                break;
            }

            response.push_str(&line);
        }

        Ok(response)
    }
}
