use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error)
}