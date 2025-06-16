use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, StdinLock},
};

pub enum ArgumentError {
    InvalidNumberOfArguments(usize),
    FileNotFound(String, std::io::Error),
}

#[derive(Debug)]
pub enum ProcessingInput {
    File(BufReader<File>),
    Stdin(BufReader<StdinLock<'static>>),
}

impl ProcessingInput {
    pub fn reader(&mut self) -> &mut dyn BufRead {
        match self {
            ProcessingInput::File(reader) => reader,
            ProcessingInput::Stdin(reader) => reader,
        }
    }
}


#[derive(Debug)]
pub enum Operation {
    Process(ProcessingInput),
    PrintHelp,
}

pub fn get_operation() -> Result<Operation, ArgumentError> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "-h") {
        return Ok(Operation::PrintHelp);
    }

    match args.as_slice() {
        [] => {
            let reader: BufReader<StdinLock<'static>> = BufReader::new(std::io::stdin().lock());
            Ok(Operation::Process(ProcessingInput::Stdin(reader)))
        }
        [filename] => {
            let file: File = File::open(filename).map_err(|error: std::io::Error| {
                ArgumentError::FileNotFound(filename.to_owned(), error)
            })?;
            let buffered_reader: BufReader<File> = BufReader::new(file);
            Ok(Operation::Process(ProcessingInput::File(buffered_reader)))
        }
        _ => Err(ArgumentError::InvalidNumberOfArguments(args.len())),
    }
}

pub fn print_help_message() {
    println!(
        "Usage:
    \t\txxd [-h] [file]
    Options:
    \t\t-h\t\tprint this summary."
    );
}
