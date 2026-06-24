fn main() {
    if let Err(err) = glas::run() {
        if let Some(partial) = err.downcast_ref::<glas::PartialFailure>() {
            for (target, error) in &partial.errors {
                eprintln!("glas: {}: {}", target.display(), error);
            }
            std::process::exit(1);
        } else {
            eprintln!("glas: {err:#}");
            std::process::exit(2);
        }
    }
}
