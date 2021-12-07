use super::crab::Crab;
use std::{io::Error, path::Path};

use crate::utils::read_lines;

pub fn read_crabs(file_path: &Path) -> Result<Vec<Crab>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;

        values = ip
            .split(',')
            .map(|x| Crab::new(x.parse::<i32>().unwrap()))
            .collect();
    }

    Ok(values)
}
