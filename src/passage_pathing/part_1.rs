use super::is_big_cave;
use std::collections::{HashMap, HashSet};

pub fn find_possible_paths(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    let mut current_path = vec![];
    let mut paths = vec![];

    visit(
        "start",
        "end",
        graph,
        &mut current_path,
        &mut visited,
        &mut paths,
    );

    paths.len()
}

fn visit<'a>(
    src: &'a str,
    end: &'a str,
    graph: &'a HashMap<String, Vec<String>>,
    current_path: &mut Vec<&'a str>,
    visited: &mut HashSet<&'a str>,
    paths: &mut Vec<Vec<&'a str>>,
) {
    current_path.push(src);
    if !is_big_cave(src) {
        visited.insert(src);
    }
    if src == end {
        paths.push(current_path.clone());
    } else if let Some(dests) = graph.get(src) {
        for dest in dests {
            if !visited.contains(dest.as_str()) {
                visit(dest.as_str(), end, graph, current_path, visited, paths);
            }
        }
    }
    visited.remove(src);
    current_path.pop();
}
