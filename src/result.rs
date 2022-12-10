use std::io::Error as IoError;

use exitcode;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    KeyboardInterrupt,
    OutOfLineNumbers,
    UnexpectedCharacter { character: char, position: usize },
    UnexpectedEof,
    UnterminatedString { string: String, position: usize },
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match &self {
            Error::IoError(_) => exitcode::IOERR,
            Error::KeyboardInterrupt => exitcode::OK,
            Error::OutOfLineNumbers => exitcode::SOFTWARE,
            Error::UnexpectedCharacter {
                character: _,
                position: _,
            } => exitcode::USAGE,
            Error::UnexpectedEof => exitcode::USAGE,
            Error::UnterminatedString {
                string: _,
                position: _,
            } => exitcode::USAGE,
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
