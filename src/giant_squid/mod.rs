use crate::giant_squid::board::Board;
use std::path::Path;

mod board;
mod input;
mod part_1;
mod part_2;

pub fn solve(file_path: &Path) {
    let (values, matrices) = input::read_values::<5>(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let boards: Vec<_> = matrices.iter().map(Board::new).collect();

    let score = part_1::get_winning_board_score(&values, boards.clone());
    println!("Giant Squid Part 1 Score: {}", score);

    let score = part_2::get_last_winning_board_score(&values, boards);
    println!("Giant Squid Part 2 Score: {}", score);
}
