mod arguments;
mod formattings;

use arguments::Operation;
use std::io::{self, BufRead};
use thiserror::Error;

use crate::xxd::arguments::ProcessingInput;

#[derive(Error, Debug)]
pub enum XXDError {
    #[error("{0} arguments were given")]
    TooManyArguments(usize),

    #[error("File '{0}' was not found: {1}")]
    FileNotFound(String, io::Error),

    #[error("Failed to read from input: {0}")]
    FileReadError(io::Error),
}

fn print_xxd_output(mut processing_input: ProcessingInput) -> Result<(), XXDError> {
    let mut chunk: [u8; 16] = [0; 16];
    let reader: &mut (dyn BufRead) = processing_input.reader();

    for index in 0.. {
        let bytes_read: usize = reader
            .read(&mut chunk)
            .map_err(|err| XXDError::FileReadError(err))?;
        if bytes_read == 0 {
            break;
        }

        let line_index = index * 16;
        let hex_pairs = formattings::get_hex_section(&chunk[..bytes_read]);
        let hex_str = unsafe {
            std::str::from_utf8_unchecked(hex_pairs.as_ref())
        };
        let ascii_section: [u8; 16] = formattings::get_ascii_section(&chunk[..bytes_read]);
        let ascii_str = unsafe {
            std::str::from_utf8_unchecked(&ascii_section)
        };

        print!("{line_index:08x}: {hex_str} {ascii_str}\n");
    };
    Ok(())
}

pub fn run_xxd() -> Result<(), XXDError> {
    let operation: Operation =
        arguments::get_operation().map_err(|error: arguments::ArgumentError| match error {
            arguments::ArgumentError::InvalidNumberOfArguments(x) => XXDError::TooManyArguments(x),
            arguments::ArgumentError::FileNotFound(path, error) => {
                XXDError::FileNotFound(path, error)
            }
        })?;

    match operation {
        Operation::PrintHelp => arguments::print_help_message(),
        Operation::Process(input_operation) => print_xxd_output(input_operation)?,
    };

    Ok(())
}
