fn main() {
    if let Err(err) = glas::run() {
        eprintln!("glas: {err:#}");
        std::process::exit(1);
    }
}
