use std::io::{self, Read};
use std::fs::File;

pub fn get_stdin_contents() -> Result<Vec<u8>, io::Error> {
    let mut contents: Vec<u8> = vec![];

    io::stdin().read_to_end(&mut contents)?;

    Ok(contents)
}

pub fn get_file_contents(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file: File = File::open(path)?;
    let mut contents: Vec<u8> = vec![];

    file.read_to_end(&mut contents)?;

    Ok(contents)
}
