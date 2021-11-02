fn main() {
    if let Err(e) = rust_wc::parse_args().and_then(rust_wc::run) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
