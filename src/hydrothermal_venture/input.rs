use super::Vent;
use std::{io::Error, path::Path};

use crate::utils::read_lines;

pub fn read_vents(file_path: &Path) -> Result<Vec<Vent>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;

        let mut buf = [0; 4];

        ip.split(&" -> ")
            .flat_map(|s| s.split(','))
            .map(|x| x.parse::<i32>().unwrap())
            .enumerate()
            .for_each(|(i, val)| {
                buf[i] = val;
            });

        values.push(Vent::new((buf[0], buf[1]), (buf[2], buf[3])));
    }

    Ok(values)
}
