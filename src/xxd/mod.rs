
mod arguments;
mod formattings;
mod input;

use std::io;
use arguments::{Operation, InputOperation};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum XXDError {
    #[error("{0} arguments were given")]
    TooManyArguments(usize),

    #[error("{0}: {1}")]
    GetFileContentsFailed(String, io::Error),

    #[error("{0}")]
    GetStdinContentsFailed(io::Error),
}

fn get_full_output(arr: &[u8]) -> String {
    let mut output: String = String::new();

    for (index, chunk) in arr.chunks(16).enumerate() {
        let line_index: String = formattings::get_line_index(index);
        let pairs: String = formattings::get_pairs_section(chunk);
        let text: String = formattings::get_ascii_section(chunk);

        output += &format!("{line_index}: {pairs:40} {text:}\n");
    }

    output
}


pub fn run_xxd() -> Result<String, XXDError> {
    let operation: Operation = arguments::get_operation().map_err(
        |error: arguments::ArgumentError| match error {
            arguments::ArgumentError::InvalidNumberOfArguments(x) => XXDError::TooManyArguments(x)
        }
    )?;

    let contents: Vec<u8> = match operation {
        Operation::PrintHelp => return Ok(arguments::get_help_message()),

        Operation::ProcessingOperation(input_operation) => match input_operation {
            InputOperation::File(path) => input::get_file_contents(path.as_ref()).map_err(
                |error| XXDError::GetFileContentsFailed(path, error)
            ),

            InputOperation::Stdin => input::get_stdin_contents().map_err(
                |error| XXDError::GetStdinContentsFailed(error)
            )
        }
    }?;

    Ok(get_full_output(&contents))
}
