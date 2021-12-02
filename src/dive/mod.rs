use std::path::Path;

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

mod input;
mod part_1;
mod part_2;

pub fn solve(file_path: &Path) {
    let movements = input::read_movements(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let (horizontal, depth) = part_1::find_position(&movements);
    println!(
        "Dive Part 1: horizontal: {}, depth: {}, answer: {}",
        horizontal,
        depth,
        horizontal * depth
    );

    let (horizontal, depth) = part_2::find_position(&movements);
    println!(
        "Dive Part 2: horizontal: {}, depth: {}, answer: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}
