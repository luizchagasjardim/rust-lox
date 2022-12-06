use std::io::Error as IoError;

use exitcode;

pub enum Error {
    IoError(IoError),
    KeyboardInterrupt,
}

impl Error {
    fn exit_code(&self) -> i32 {
        match &self {
            Error::IoError(_) => exitcode::IOERR,
            Error::KeyboardInterrupt => exitcode::OK,
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn exit_code<T>(res: Result<T>) -> i32 {
    match res {
        Ok(_) => exitcode::OK,
        Err(e) => e.exit_code(),
    }
}
