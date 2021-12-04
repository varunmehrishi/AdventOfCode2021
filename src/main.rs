use std::env;
use std::path::Path;

mod binary_diagnostic;
mod dive;
mod giant_squid;
mod sonar_sweep;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage ./aoc path_to_directory_containing_input_files");
    }

    let base = Path::new(&args[1]);

    sonar_sweep::solve(&base.join("sonar_sweep_input.txt"));
    dive::solve(&base.join("dive_input.txt"));
    binary_diagnostic::solve(&base.join("binary_diagnostic_input.txt"));
    giant_squid::solve(&base.join("giant_squid_input.txt"));
}
