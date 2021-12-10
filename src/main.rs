use std::env;
use std::path::Path;

mod binary_diagnostic;
mod dive;
mod giant_squid;
mod hydrothermal_venture;
mod lanternfish;
mod seven_segment;
mod smoke_basin;
mod sonar_sweep;
mod syntax_scoring;
mod treachery_whales;
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
    hydrothermal_venture::solve(&base.join("hydrothermal_venture_input.txt"));
    lanternfish::solve(&base.join("lanternfish_input.txt"));
    treachery_whales::solve(&base.join("treachery_whales_input.txt"));
    seven_segment::solve(&base.join("seven_segment_input.txt"));
    smoke_basin::solve::<100, 100>(&base.join("smoke_basin_input.txt"));
    syntax_scoring::solve(&base.join("syntax_scoring_input.txt"));
}
