fn main() {
    if let Err(e) = headson::get_args().and_then(headson::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
