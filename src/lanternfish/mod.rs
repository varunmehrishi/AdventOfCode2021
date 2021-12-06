use crate::lanternfish::fish::Fish;
use core::panic;
use std::{collections::HashMap, path::Path};

mod fish;
mod input;
mod part_1;
mod part_2;

fn age_fishes(fishes: HashMap<Fish, i64>) -> HashMap<Fish, i64> {
    let mut new_fishes: HashMap<Fish, i64> = HashMap::new();
    for (fish, count) in fishes {
        let mut fish = fish.clone();
        if let Some(fish) = fish.age() {
            *new_fishes.entry(fish).or_insert(0) += count;
        }
        *new_fishes.entry(fish).or_insert(0) += count;
    }
    new_fishes
}

pub fn solve(file_path: &Path) {
    let fishes = input::read_fishes(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let uniq_fishes = {
        let mut uniq_fishes: HashMap<Fish, i64> = HashMap::new();
        for fish in &fishes {
            *uniq_fishes.entry(fish.clone()).or_insert(0) += 1;
        }
        uniq_fishes
    };

    let count = part_1::age(uniq_fishes.clone(), 80);
    println!("Lanternfish Part 1 answer: {}", count);

    let count = part_2::age(uniq_fishes, 256);
    println!("Lanternfish Part 2 answer: {}", count);
}
