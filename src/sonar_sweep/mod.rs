use std::path::Path;

mod input;
mod part_1;
mod part_2;

pub fn solve(file_path: &Path) {
    let values = input::read_values(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let increments = part_1::find_increments(&values);
    println!("Sonar Sweep Part 1: increments {}", increments);

    let increments = part_2::find_increments(&values);
    println!("Sonar Sweep Part 2: increments {}", increments);
}
