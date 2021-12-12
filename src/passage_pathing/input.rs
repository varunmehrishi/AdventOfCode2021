// use super::octopus::Octopus;
use crate::utils::read_lines;
use std::{collections::HashMap, io::Error, path::Path};

pub fn read_graph(file_path: &Path) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut graph = HashMap::new();
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        if let Some((src, dest)) = ip.split_once('-') {
            graph
                .entry(src.to_owned())
                .or_insert_with(Vec::new)
                .push(dest.to_owned());
            graph
                .entry(dest.to_owned())
                .or_insert_with(Vec::new)
                .push(src.to_owned());
        }
    }

    Ok(graph)
}
