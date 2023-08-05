use std::{
    fs::{write, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::{Parser, Subcommand, ValueEnum};
use error::AssemblerError;
use instruction::Instruction;
use parser::Line;
use tracing::{debug, metadata::LevelFilter};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::{parser::parse_line, process::proccess_instructions};

mod directive;
mod error;
mod instruction;
mod parser;
mod process;
mod validation;

#[derive(Parser)]
#[command(
    author = "Kyle Gagnon",
    version = "0.1.0",
    about = "Parses 6502 instruction set into JSON or a byte array. Optionally assembles the file."
)]
struct Cli {
    /// The input assembly file; Required
    #[arg(short, long, value_name = "INPUT")]
    input: PathBuf,

    /// The output file (with extension)
    #[arg(short, long, value_name = "OUTPUT")]
    output: PathBuf,

    /// The level of verbosity to use
    #[arg(short, long)]
    verbose: Option<VerboseLevels>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Assembles the file to a binary
    Assemble,
    /// Writes the assembly file to a JSON format
    Json,
    /// Generates the full ROM for the NES ROM
    NES,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum VerboseLevels {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub fn main() {
    let cli = Cli::parse();

    let level = match cli.verbose {
        Some(VerboseLevels::Trace) => LevelFilter::TRACE,
        Some(VerboseLevels::Debug) => LevelFilter::DEBUG,
        Some(VerboseLevels::Info) => LevelFilter::INFO,
        Some(VerboseLevels::Warn) => LevelFilter::WARN,
        Some(VerboseLevels::Error) => LevelFilter::ERROR,
        None => LevelFilter::OFF,
    };

    let filter = EnvFilter::from_default_env().add_directive(level.into());

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let reader = match open_file(&cli.input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}: error - {}", cli.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    };
    let lines = match parse_file(reader) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("{}: error - {}", cli.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    };

    match &cli.command {
        Commands::Assemble => {
            assemble(lines, &cli);
        }
        Commands::Json => {
            json(lines, &cli);
        }
        Commands::NES => {
            nes(lines, &cli);
        }
    }
}

pub fn open_file(file: &PathBuf) -> Result<Vec<String>, AssemblerError> {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(e) => return Err(AssemblerError::IOError(e.to_string())),
    };
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|l| l.expect("Could not parse file!"))
        .collect())
}

pub fn parse_file(str_lines: Vec<String>) -> Result<Vec<(Line, usize)>, AssemblerError> {
    let mut lines: Vec<(Line, usize)> = Vec::new();

    for (i, line) in str_lines.iter().enumerate() {
        let line_num = i + 1;
        match parse_line(line, line_num) {
            Ok(line) => lines.push((line, line_num)),
            Err(e) => return Err(e),
        }
    }

    Ok(lines)
}

pub fn to_bytes(lines: Vec<Instruction>) -> Result<Vec<u8>, AssemblerError> {
    let mut bytes: Vec<u8> = Vec::new();

    for instr in lines {
        match instr.to_bytes() {
            Ok(bytes_res) => bytes.extend(bytes_res),
            Err(e) => return Err(e),
        };
    }

    Ok(bytes)
}

fn add_padding(mut bytes: Vec<u8>, start_pos: u16) -> Result<Vec<u8>, AssemblerError> {
    // Ensure the assembled program doesn't exceed the PRG ROM size
    if bytes.len() > 0xFFC {
        return Err(AssemblerError::ProgramTooLarge);
    }

    // Add padding to reach the reset vector
    let padding = vec![0; 0xFFC - bytes.len()];
    bytes.extend(padding);

    // Split the start_pos into low and high bytes
    let hi = (start_pos >> 8) as u8;
    let lo = (start_pos & 0xff) as u8;

    // Write the reset vector
    bytes.push(lo);
    bytes.push(hi);

    // Add the reset of the interrupt vectors (NMI, IRQ/BRK)
    // Set to zero for now, adjust when needed
    bytes.extend(vec![0; 4]);

    Ok(bytes)
}

fn write_bytes_to_file(file: &PathBuf, bytes: &Vec<u8>) -> Result<(), AssemblerError> {
    match write(file, bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(AssemblerError::IOError(e.to_string())),
    }
}

fn assemble(lines: Vec<(Line, usize)>, args: &Cli) {
    let instructions = match proccess_instructions(lines) {
        Ok((instrs, _)) => instrs,
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    };

    let bytes = match to_bytes(instructions) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    };

    debug!(
        "{}",
        bytes
            .iter()
            .map(|byte| format!("0x{:02X}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    );

    match write_bytes_to_file(&args.output, &bytes) {
        Ok(_) => println!(
            "Assembled {} bytes to {}",
            bytes.len(),
            &args.output.to_string_lossy()
        ),
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    }
}

fn json(lines: Vec<(Line, usize)>, args: &Cli) {
    let file = match File::create(&args.output) {
        Ok(file) => file,
        Err(_) => {
            eprintln!(
                "{}: error - Failed to open file",
                args.input.to_str().unwrap()
            );
            std::process::exit(1);
        }
    };
    match serde_json::to_writer_pretty(file, &lines) {
        Ok(_) => println!(
            "Successfully wrote the JSON to {}",
            &args.output.to_string_lossy()
        ),
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    }
}

fn nes(lines: Vec<(Line, usize)>, args: &Cli) {
    let (instructions, start_pos) = match proccess_instructions(lines) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    };

    let bytes = match to_bytes(instructions) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    };

    let padded_bytes = match add_padding(bytes, start_pos as u16) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    };

    match write_bytes_to_file(&args.output, &padded_bytes) {
        Ok(_) => println!(
            "Assembled {} bytes to {}",
            padded_bytes.len(),
            &args.output.to_string_lossy()
        ),
        Err(e) => {
            eprintln!("{}: error - {}", args.input.to_str().unwrap(), e);
            std::process::exit(1);
        }
    }
}
