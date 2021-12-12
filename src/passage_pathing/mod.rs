use core::panic;
use std::path::Path;

mod input;
mod part_1;
mod part_2;

pub fn solve(file_path: &Path) {
    let graph = input::read_graph(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let paths = part_1::find_possible_paths(&graph);
    println!("Passage Pathing Part 1 answer: {}", paths);

    let paths = part_2::find_possible_paths(&graph);
    println!("Passage Pathing Part 2 answer: {}", paths);
}

fn is_big_cave(cave: &str) -> bool {
    cave.chars().all(|x| char::is_ascii_uppercase(&x))
}
