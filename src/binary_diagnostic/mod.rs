use core::panic;
use std::path::Path;

mod input;
mod part_1;
mod part_2;

pub fn solve(file_path: &Path) {
    let values = input::read_values::<12>(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let counts = get_counts(&values);

    let (gamma, epsilon) = part_1::get_gamma_epsilon(&counts, (values.len() as u32) / 2);
    let power_consumption = gamma * epsilon;
    println!("Binary Diagnostic Part 1 Answer: {} ", power_consumption);

    let (oxygen, co2) = part_2::get_oxygen_co2_rating(&values);
    let life_support_rating = oxygen * co2;
    println!("Binary Diagnostic Part 2 Answer: {} ", life_support_rating);
}

fn get_counts<const T: usize>(values: &[[u32; T]]) -> [u32; T] {
    let mut counts = [0; T];

    for value in values {
        for i in 0..value.len() {
            counts[i] += value[i];
        }
    }

    counts
}

fn get_count_at_index<const T: usize>(values: &[[u32; T]], i: usize) -> u32 {
    let mut count = 0;
    assert!(i < T);

    for value in values {
        count += value[i];
    }

    count
}
