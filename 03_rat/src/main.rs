fn main() {
    if let Err(e) = rat::get_args().and_then(rat::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
