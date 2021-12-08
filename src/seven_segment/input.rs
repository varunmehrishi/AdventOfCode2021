use crate::utils::read_lines;
use std::{io::Error, path::Path};

type Segment = ([String; 10], [String; 4]);

pub fn read_values(file_path: &Path) -> Result<Vec<Segment>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;

        if let Some((left, right)) = ip.split_once(" | ") {
            values.push((read_n_strings::<10>(left), read_n_strings::<4>(right)));
        }
    }

    Ok(values)
}

fn read_n_strings<const N: usize>(s: &str) -> [String; N] {
    const EMPTY_STRING: String = String::new();
    let mut buffer: [String; N] = [EMPTY_STRING; N];
    s.split_whitespace()
        .take(N)
        .enumerate()
        .for_each(|(i, s)| buffer[i] = trim_and_sort::<8>(s));
    buffer
}

fn trim_and_sort<const N: usize>(s: &str) -> String {
    let mut buffer: [char; N] = [' '; N];
    let mut count = 0;
    s.trim().chars().into_iter().for_each(|c| {
        buffer[count] = c;
        count += 1;
    });

    buffer[0..count].sort_unstable();
    buffer[0..count].iter().collect()
}
