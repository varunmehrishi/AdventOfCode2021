use core::panic;
use std::path::Path;

mod input;
mod part_1;
mod part_2;
mod point;

pub fn solve<const N: usize, const M: usize>(file_path: &Path) {
    let matrix = input::read_matrix::<N, M>(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let score = part_1::get_low_point_score::<N, M>(matrix);
    println!("Smoke Basin Part 1 answer: {}", score);

    let score = part_2::get_largest_basins_score::<N, M>(matrix);
    println!("Smoke Basin Part 2 answer: {}", score);
}
