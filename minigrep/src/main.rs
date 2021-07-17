use std::process;
use std::env;

use minigrep::{Config};

fn main() {
    // read arguments from the command line - panics for invalid unicode
    let args: Vec<String> = env::args().collect();

    // The first arg is file name, we will save two args we need
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        // eprintln! writes to stderr
        eprintln!("Problem parsing arguments: {}", err);
        // note that this works as there's NO alternate assignment, otherwise we'd get
        // compiler method for mismatched types!
        process::exit(1);
    });

    // Let's use if let pattern as we don't care for Result(())
    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
