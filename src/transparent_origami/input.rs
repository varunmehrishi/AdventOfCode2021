use super::{FoldAlong, Point};
use crate::utils::read_lines;
use std::{collections::HashSet, io::Error, path::Path};

pub(super) fn read_points_and_folds(
    file_path: &Path,
) -> Result<(HashSet<Point>, Vec<FoldAlong>), Error> {
    let mut points = HashSet::new();
    let mut folds = vec![];
    let lines = read_lines(file_path)?;

    for line in lines {
        let ip = line?;
        if let Some((x, y)) = ip.split_once(',') {
            points.insert(Point::new(x.parse().unwrap(), y.parse().unwrap()));
        }

        if ip.contains("fold along") {
            ip.split_whitespace()
                .take(3)
                .skip(2)
                .filter_map(|fold| fold.split_once('='))
                .for_each(|fold| match fold {
                    ("x", v) => folds.push(FoldAlong::X(v.parse().unwrap())),
                    ("y", v) => folds.push(FoldAlong::Y(v.parse().unwrap())),
                    _ => unreachable!("unreachable"),
                })
        }
    }

    Ok((points, folds))
}
