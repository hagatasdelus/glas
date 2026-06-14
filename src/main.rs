fn main() {
    if let Err(err) = glas::run() {
        if err.is::<glas::PartialFailure>() {
            std::process::exit(1);
        } else {
            eprintln!("glas: {err:#}");
            std::process::exit(1);
        }
    }
}
