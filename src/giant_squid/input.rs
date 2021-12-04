use core::panic;
use std::{io::Error, path::Path};

use crate::utils::read_lines;
type Matrix<const T: usize> = [[u32; T]; T];

pub fn read_values<const T: usize>(file_path: &Path) -> Result<(Vec<u32>, Vec<Matrix<T>>), Error> {
    let mut lines = read_lines(file_path)?;

    let values = {
        if let Some(first_line) = lines.next() {
            first_line?
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        } else {
            panic!("Unable to read first line {:?}", file_path);
        }
    };

    lines.next(); // Skip empty line

    let mut matrices = vec![];
    let mut buffer = [[0; T]; T];
    let mut buf_line = 0;
    for line in lines {
        let ip = line?;

        if !ip.trim().is_empty() {
            ip.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(i, val)| {
                    buffer[buf_line][i] = val;
                });

            buf_line += 1;

            if buf_line == T {
                matrices.push(buffer);
                buf_line = 0;
            }
        }
    }

    Ok((values, matrices))
}
