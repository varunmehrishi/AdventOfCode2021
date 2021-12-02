use std::{io::Error, path::Path};

use super::Command;
use crate::utils::read_lines;

pub(super) fn read_movements(file_path: &Path) -> Result<Vec<Command>, Error> {
    let mut movements = vec![];

    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        let op: Vec<_> = ip.split_whitespace().collect();

        assert!(op.len() == 2);

        let magnitude: i64 = op[1].parse().unwrap();
        let movement = match op[0] {
            "forward" => Command::Forward(magnitude),
            "up" => Command::Up(magnitude),
            "down" => Command::Down(magnitude),
            _ => panic!("Unmatched input"),
        };

        movements.push(movement);
    }

    Ok(movements)
}
