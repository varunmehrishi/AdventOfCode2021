use std::collections::HashMap;

use super::age_fishes;
use super::fish::Fish;

pub fn age(mut fishes: HashMap<Fish, i64>, n: usize) -> i64 {
    for _ in 0..n {
        fishes = age_fishes(fishes);
    }

    fishes.values().sum::<i64>()
}
