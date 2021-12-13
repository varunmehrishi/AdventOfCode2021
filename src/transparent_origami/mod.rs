use core::panic;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::Path;

mod input;
mod part_1;
mod part_2;

#[derive(Debug)]
enum FoldAlong {
    X(i32),
    Y(i32),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, std::cmp::Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn fold(&self, fold: &FoldAlong) -> Option<Point> {
        match fold {
            FoldAlong::X(f) => match f.cmp(&self.x) {
                Ordering::Less => Some(Point::new(2 * f - self.x, self.y)),
                Ordering::Equal => None,
                Ordering::Greater => Some(*self),
            },
            FoldAlong::Y(f) => match f.cmp(&self.y) {
                Ordering::Less => Some(Point::new(self.x, 2 * f - self.y)),
                Ordering::Equal => None,
                Ordering::Greater => Some(*self),
            },
        }
    }
}

fn fold(points: &HashSet<Point>, fold: &FoldAlong) -> HashSet<Point> {
    points.iter().filter_map(|p| p.fold(fold)).collect()
}

pub fn solve(file_path: &Path) {
    let (points, folds) = input::read_points_and_folds(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let answer = part_1::count_new_points(&points, folds.iter().take(1).next().unwrap());
    println!("Transparent Origami Part 1: {}", answer);

    println!("Transparent Origami Part 2: ");
    part_2::plot_new_points(&points, &folds);
}
