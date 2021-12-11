use core::panic;
use std::path::Path;

mod input;
mod octopus;
mod part_1;
mod part_2;

pub fn solve<const N: usize, const M: usize>(file_path: &Path) {
    let matrix = input::read_matrix::<N, M>(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let flashes = part_1::step::<N, M>(matrix, 100);
    println!("Dumbo Octopus Part 1 answer: {}", flashes);

    let steps = part_2::find_steps::<N, M>(matrix);
    println!("Dumbo Octopus Part 2 answer: {}", steps);
}
