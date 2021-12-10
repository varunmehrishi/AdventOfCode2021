use crate::utils::read_lines;
use std::{io::Error, path::Path};

pub fn read_values(file_path: &Path) -> Result<Vec<String>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        values.push(ip);
    }

    Ok(values)
}
