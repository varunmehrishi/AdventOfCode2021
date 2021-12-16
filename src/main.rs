#![feature(map_first_last)]
use std::env;
use std::path::Path;

mod binary_diagnostic;
mod chiton;
mod dive;
mod dumbo_octopus;
mod extended_polymerization;
mod giant_squid;
mod hydrothermal_venture;
mod lanternfish;
mod passage_pathing;
mod seven_segment;
mod smoke_basin;
mod sonar_sweep;
mod syntax_scoring;
mod transparent_origami;
mod treachery_whales;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage ./aoc path_to_directory_containing_input_files");
        return;
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
    dumbo_octopus::solve::<10, 10>(&base.join("dumbo_octopus_input.txt"));
    passage_pathing::solve(&base.join("passage_pathing_input.txt"));
    transparent_origami::solve(&base.join("transparent_origami_input.txt"));
    extended_polymerization::solve(&base.join("extended_polymerization_input.txt"));
    chiton::solve::<100, 100>(&base.join("chiton_input.txt"));
}
