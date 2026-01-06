use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum EnsoError {
    Io(std::io::Error),
    Protocol(String),
}

impl From<std::io::Error> for EnsoError {
    fn from(e: std::io::Error) -> Self {
        EnsoError::Io(e)
    }
}

