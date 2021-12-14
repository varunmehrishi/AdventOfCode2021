use crate::utils::read_lines;
use std::{collections::HashMap, io::Error, path::Path};

type Reactions = HashMap<(char, char), char>;

pub fn read_template_and_rules(file_path: &Path) -> Result<(Vec<char>, Reactions), Error> {
    let mut template = vec![];
    let mut rules = HashMap::new();
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        if let Some((x, y)) = ip.split_once(" -> ") {
            let mut chars = x.chars().take(2);
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();
            rules.insert((first, second), y.chars().take(1).next().unwrap());
        } else if !ip.is_empty() {
            template = ip.chars().collect();
        }
    }

    Ok((template, rules))
}
