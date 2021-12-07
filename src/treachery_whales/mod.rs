use core::panic;
use std::path::Path;

mod crab;
mod input;
mod part_1;
mod part_2;

pub fn solve(file_path: &Path) {
    let crabs = input::read_crabs(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let cost = part_1::compute_min_cost(&crabs);
    println!("The Treachery of Whales Part 1 answer: {}", cost);

    let cost = part_2::compute_min_cost(&crabs);
    println!("The Treachery of Whales Part 2 answer: {}", cost);
}
