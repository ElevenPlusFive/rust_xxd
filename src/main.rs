mod xxd;

fn main() {
    let xxd_result = xxd::run_xxd();
    if let Err(e) = xxd_result {
        eprintln!("XXD Error: {}", e);
        std::process::exit(1);
    }
}
