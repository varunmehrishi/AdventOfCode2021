use super::fish::Fish;
use std::{io::Error, path::Path};

use crate::utils::read_lines;

pub fn read_fishes(file_path: &Path) -> Result<Vec<Fish>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;

        values = ip
            .split(',')
            .map(|x| Fish::new(x.parse::<i32>().unwrap()))
            .collect();
    }

    Ok(values)
}
