use std::{io::Error, path::Path};

use crate::utils::read_lines;

pub fn read_values<const T: usize>(file_path: &Path) -> Result<Vec<[u32; T]>, Error> {
    let mut values = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        // values.push(ip);
        let mut arr = [0; T];
        ip.chars()
            .enumerate()
            .for_each(|(i, x): (usize, char)| match x {
                '0' => {
                    arr[i] = 0;
                }
                '1' => {
                    arr[i] = 1;
                }
                _ => panic!("Unexpected char"),
            });
        values.push(arr);
    }

    Ok(values)
}
