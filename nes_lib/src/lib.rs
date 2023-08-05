use std::path::PathBuf;

use error::EmulatorError;

pub mod cpu;
pub mod error;
pub mod instructions;

pub fn open_bin_file(file: &PathBuf) -> Result<Vec<u8>, EmulatorError> {
    let bytes = std::fs::read(file)?;
    Ok(bytes)
}