use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AssemblerError {
    #[error("Parse error: {0}")]
    ParseErrorNom(String),

    #[error("Parse error at line {line}: {msg}")]
    ParseError {
        msg: String,
        line: usize,
    },

    #[error("Parse error - {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("I/O error: {0}")]
    IOError(String),

    #[error("Invalid operand: {0}")]
    InvalidOperand(String),

    #[error("Incomplete input")]
    IncompleteInput,

    #[error("Invalid opcode at line: {line}: {msg}")]
    InvalidOpCode {
        msg: String,
        line: usize
    },

    #[error("Unable to convert opcode to u8: {0}")]
    OpCodeConversionError(String),

    #[error("Operand address out of range: {0}")]
    OperandOutOfRange(String),

    #[error("Invalid Directive: {0}")]
    InvalidDirective(String),

    #[error("Invalid label at line: {line}: {msg}")]
    InvalidLabel{
        msg: String,
        line: usize
    },

    #[error("Program too large")]
    ProgramTooLarge
}

impl From<nom::Err<nom::error::Error<&str>>> for AssemblerError {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => AssemblerError::IncompleteInput,
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                AssemblerError::ParseErrorNom(e.input.to_string())
            }
        }
    }
}