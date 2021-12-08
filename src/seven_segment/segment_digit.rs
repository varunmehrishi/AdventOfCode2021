use super::Digit;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct SegmentDigit {
    pub wires: String,
    pub value: Option<Digit>,
}

impl SegmentDigit {
    pub fn new(wires: String, len_to_digit: &HashMap<usize, Vec<Digit>>) -> Self {
        let probable_values = len_to_digit.get(&wires.len()).unwrap();
        let is_determined = probable_values.len() == 1usize;
        let value: Option<Digit> = if is_determined {
            Some(probable_values[0])
        } else {
            None
        };
        SegmentDigit { value, wires }
    }

    pub fn len(&self) -> usize {
        self.wires.len()
    }

    pub fn is_determined(&self) -> bool {
        self.value.is_some()
    }

    pub fn get_intersection_count(&self, other: &Self) -> usize {
        let s1: HashSet<char> = self.wires.chars().collect();
        let s2: HashSet<char> = other.wires.chars().collect();
        s1.intersection(&s2).count()
    }

    pub fn determine(&mut self, value: Digit) {
        self.value = Some(value);
    }
}
