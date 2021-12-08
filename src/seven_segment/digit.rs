use std::collections::HashSet;
#[derive(Debug, Hash, std::cmp::Eq, PartialEq, Copy, Clone)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    pub const VALUES: [Self; 10] = [
        Self::Zero,
        Self::One,
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
    ];

    pub fn get_chars(digit: Digit) -> HashSet<char> {
        match digit {
            Digit::Zero => "abcefg".chars().collect(),
            Digit::One => "cf".chars().collect(),
            Digit::Two => "acdeg".chars().collect(),
            Digit::Three => "acdfg".chars().collect(),
            Digit::Four => "bcdf".chars().collect(),
            Digit::Five => "abdfg".chars().collect(),
            Digit::Six => "abdefg".chars().collect(),
            Digit::Seven => "acf".chars().collect(),
            Digit::Eight => "abcdefg".chars().collect(),
            Digit::Nine => "abcdfg".chars().collect(),
        }
    }

    pub fn get_value(digit: Digit) -> i32 {
        match digit {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}
