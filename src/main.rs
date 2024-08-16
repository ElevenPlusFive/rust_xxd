mod xxd;

fn main() {
    match xxd::run_xxd() {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("xxd: {error}")
    };
}
