use std::env;

pub enum ArgumentError {
    InvalidNumberOfArguments(usize)
}


#[derive(Debug)]
pub enum InputOperation {
    File(String),
    Stdin
}


#[derive(Debug)]
pub enum Operation {
    ProcessingOperation(InputOperation),
    PrintHelp
}


pub fn get_operation() -> Result<Operation, ArgumentError>{
    let mut args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "-h") {
        return Ok(Operation::PrintHelp);
    }

    match args.len() {
        1 => Ok(Operation::ProcessingOperation(InputOperation::Stdin)),
        2 => Ok(Operation::ProcessingOperation(InputOperation::File(args.remove(1)))),
        x => Err(ArgumentError::InvalidNumberOfArguments(x))
    }
}

pub fn get_help_message() -> String {
    "Usage:
    \t\txxd [-h] [file]
    Options:
    \t\t-h\t\tprint this summary.".to_string()
}