use crate::hydrothermal_venture::vent::Vent;
use core::panic;
use std::path::Path;

mod floor;
mod input;
mod part_1;
mod part_2;
mod vent;

pub fn solve(file_path: &Path) {
    let vents = input::read_vents(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let intersections = part_1::get_straight_intersection_count(&vents);
    println!("Hydrothermal Venture Part 1 answer: {}", intersections);

    let intersections = part_2::get_intersection_count(&vents);
    println!("Hydrothermal Venture Part 2 answer: {}", intersections);
}
