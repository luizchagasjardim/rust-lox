use crate::object::Error as ObjectError;
use std::io::Error as IoError;

use exitcode;

#[derive(Debug)]
pub enum Error {
    EvaluationError(ObjectError),
    ExpectedEndOfBlock,
    ExpectedEndOfExpression,
    ExpectedExpression { position: usize },
    InvalidAssignmentTarget,
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
            Error::EvaluationError(_) => exitcode::USAGE,
            Error::ExpectedEndOfBlock => exitcode::USAGE,
            Error::ExpectedEndOfExpression => exitcode::USAGE,
            Error::ExpectedExpression { .. } => exitcode::USAGE,
            Error::IoError(_) => exitcode::IOERR,
            Error::KeyboardInterrupt => exitcode::OK,
            Error::InvalidAssignmentTarget => exitcode::USAGE,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockOsError;
    impl std::fmt::Display for MockOsError {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
    impl std::error::Error for MockOsError {}
    unsafe impl Sync for MockOsError {}
    unsafe impl Send for MockOsError {}

    #[test]
    fn error_from_io_error() {
        let io_error = IoError::new(std::io::ErrorKind::NotFound, MockOsError {});
        let error: Error = io_error.into();
        assert!(matches!(error, Error::IoError(_)));
    }

    mod exit_code {
        use super::*;

        #[test]
        fn expected_expression() {
            let error = Error::ExpectedExpression { position: 0 };
            assert_eq!(error.exit_code(), exitcode::USAGE);
        }

        #[test]
        fn io_error() {
            let io_error = IoError::new(std::io::ErrorKind::NotFound, MockOsError {});
            let error: Error = io_error.into();
            assert_eq!(error.exit_code(), exitcode::IOERR);
        }

        #[test]
        fn keyboard_interrupt() {
            let error = Error::KeyboardInterrupt;
            assert_eq!(error.exit_code(), exitcode::OK);
        }

        #[test]
        fn out_of_line_numbers() {
            let error = Error::OutOfLineNumbers;
            assert_eq!(error.exit_code(), exitcode::SOFTWARE);
        }

        #[test]
        fn unexpected_character() {
            let error = Error::UnexpectedCharacter {
                character: 'a',
                position: 0,
            };
            assert_eq!(error.exit_code(), exitcode::USAGE);
        }

        #[test]
        fn unexpected_eof() {
            let error = Error::UnexpectedEof;
            assert_eq!(error.exit_code(), exitcode::USAGE);
        }

        #[test]
        fn unterminated_number() {
            let error = Error::UnterminatedNumber {
                string: "".to_string(),
                position: 0,
            };
            assert_eq!(error.exit_code(), exitcode::USAGE);
        }

        #[test]
        fn unterminated_string() {
            let error = Error::UnterminatedString {
                string: "".to_string(),
                position: 0,
            };
            assert_eq!(error.exit_code(), exitcode::USAGE);
        }

        #[test]
        fn unmatched_parenthesis() {
            let error = Error::UnmatchedParenthesis { position: 0 };
            assert_eq!(error.exit_code(), exitcode::USAGE);
        }
    }

    mod debug {
        use super::*;

        #[test]
        fn expected_expression() {
            let error = Error::ExpectedExpression { position: 0 };
            assert_eq!(format!("{:?}", error), "ExpectedExpression { position: 0 }");
        }

        #[test]
        fn io_error() {
            let io_error = IoError::new(std::io::ErrorKind::NotFound, MockOsError {});
            let error: Error = io_error.into();
            assert_eq!(
                format!("{:?}", error),
                "IoError(Custom { kind: NotFound, error: MockOsError })"
            );
        }

        #[test]
        fn keyboard_interrupt() {
            let error = Error::KeyboardInterrupt;
            assert_eq!(format!("{:?}", error), "KeyboardInterrupt");
        }

        #[test]
        fn out_of_line_numbers() {
            let error = Error::OutOfLineNumbers;
            assert_eq!(format!("{:?}", error), "OutOfLineNumbers");
        }

        #[test]
        fn unexpected_character() {
            let error = Error::UnexpectedCharacter {
                character: 'a',
                position: 0,
            };
            assert_eq!(
                format!("{:?}", error),
                "UnexpectedCharacter { character: 'a', position: 0 }"
            );
        }

        #[test]
        fn unexpected_eof() {
            let error = Error::UnexpectedEof;
            assert_eq!(format!("{:?}", error), "UnexpectedEof");
        }

        #[test]
        fn unterminated_number() {
            let error = Error::UnterminatedNumber {
                string: "".to_string(),
                position: 0,
            };
            assert_eq!(
                format!("{:?}", error),
                "UnterminatedNumber { string: \"\", position: 0 }"
            );
        }

        #[test]
        fn unterminated_string() {
            let error = Error::UnterminatedString {
                string: "".to_string(),
                position: 0,
            };
            assert_eq!(
                format!("{:?}", error),
                "UnterminatedString { string: \"\", position: 0 }"
            );
        }

        #[test]
        fn unmatched_parenthesis() {
            let error = Error::UnmatchedParenthesis { position: 0 };
            assert_eq!(
                format!("{:?}", error),
                "UnmatchedParenthesis { position: 0 }"
            );
        }
    }
}
