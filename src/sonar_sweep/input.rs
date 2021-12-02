use std::{io::Error, path::Path};

use crate::utils::read_lines;

pub fn read_values(file_path: &Path) -> Result<Vec<i32>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        let value: i32 = ip.parse().expect("Failed to parse to i32");
        values.push(value);
    }

    Ok(values)
}
