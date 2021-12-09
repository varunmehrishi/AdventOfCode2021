use super::point::Point;
use crate::utils::read_lines;
use std::{io::Error, path::Path};

pub fn read_matrix<const N: usize, const M: usize>(
    file_path: &Path,
) -> Result<[[Point; M]; N], Error> {
    let mut buffer = [[Point::new(0, 0, 0); M]; N];
    let lines = read_lines(file_path)?;

    for (row, line) in lines.take(N).enumerate() {
        let ip = line?;
        ip.chars().take(M).enumerate().for_each(|(col, c)| {
            buffer[row][col] = Point::new(char::to_digit(c, 10).unwrap(), row, col)
        });
    }

    Ok(buffer)
}
