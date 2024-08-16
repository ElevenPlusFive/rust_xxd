use std::convert::Infallible;
mod xxd;

fn main() -> Result<(), Infallible> {
    let xxd_result: Result<String, xxd::XXDError> = xxd::run_xxd();
    match xxd_result {
        Ok(output) => {
            print!("{output}");
            Ok(())
        },
        Err(error) => {
            eprintln!("xxd: {error}");
            std::process::exit(1)
        }
    }
}
