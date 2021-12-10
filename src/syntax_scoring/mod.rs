use core::panic;
use std::path::Path;

enum SyntaxStatus {
    Correct,
    Corrupt(char),
    Incomplete(Vec<char>),
}

mod input;
mod part_1;
mod part_2;

pub fn solve(file_path: &Path) {
    let values = input::read_values(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let score = part_1::find_corrupted_syntax_score(&values);
    println!("Syntax Scoring Part 1 answer: {}", score);

    let score = part_2::find_incomplete_syntax_score(&values);
    println!("Syntax Scoring Part 2 answer: {}", score);
}
