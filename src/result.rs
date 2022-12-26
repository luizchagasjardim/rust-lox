use std::io::Error as IoError;

use exitcode;

#[derive(Debug)]
pub enum Error {
    ExpectedExpression { position: usize },
    IoError(IoError),
    KeyboardInterrupt,
    OutOfLineNumbers,
    UnexpectedCharacter { character: char, position: usize },
    UnexpectedEof,
    UnterminatedNumber { string: String, position: usize },
    UnterminatedString { string: String, position: usize },
    UnmatchedParenthesis { position: usize },
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match &self {
            Error::ExpectedExpression { .. } => exitcode::USAGE,
            Error::IoError(_) => exitcode::IOERR,
            Error::KeyboardInterrupt => exitcode::OK,
            Error::OutOfLineNumbers => exitcode::SOFTWARE,
            Error::UnexpectedCharacter { .. } => exitcode::USAGE,
            Error::UnexpectedEof => exitcode::USAGE,
            Error::UnterminatedNumber { .. } => exitcode::USAGE,
            Error::UnterminatedString { .. } => exitcode::USAGE,
            Error::UnmatchedParenthesis { .. } => exitcode::USAGE,
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
