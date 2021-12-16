use crate::utils::read_lines;
use std::collections::HashMap;
use std::{io::Error, path::Path};

pub fn read_matrix<const N: usize, const M: usize>(
    file_path: &Path,
) -> Result<HashMap<(i32, i32), u32>, Error> {
    let mut map = HashMap::new();
    let lines = read_lines(file_path)?;

    for (row, line) in lines.take(N).enumerate() {
        let ip = line?;
        ip.chars().take(M).enumerate().for_each(|(col, c)| {
            map.insert((row as i32, col as i32), char::to_digit(c, 10).unwrap());
        });
    }

    Ok(map)
}
