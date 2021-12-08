use core::panic;
use digit::Digit;
use segment_digit::SegmentDigit;
use std::path::Path;

mod digit;
mod input;
mod part_1;
mod part_2;
mod segment_digit;

pub fn solve(file_path: &Path) {
    let values = input::read_values(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let count = part_1::count_uniquely_identifiable(&values);
    println!("Seven Segments Part 1 answer: {}", count);

    let sum_output_signals = part_2::get_sum_output_signals(values);
    println!("Seven Segments Part 2 answer: {}", sum_output_signals);
}
