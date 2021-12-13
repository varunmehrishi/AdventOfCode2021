use itertools::Itertools;
use itertools::MinMaxResult;

use super::{fold, FoldAlong, Point};
use std::collections::HashSet;

pub(super) fn plot_new_points(points: &HashSet<Point>, folds: &[FoldAlong]) {
    let new_points = folds
        .iter()
        .fold(points.clone(), |points, f| fold(&points, f));

    if let MinMaxResult::MinMax(mnx, mx) = new_points.iter().minmax_by_key(|&p| p.x) {
        if let MinMaxResult::MinMax(mny, my) = new_points.iter().minmax_by_key(|&p| p.y) {
            for y in mny.y..=my.y {
                for x in mnx.x..=mx.x {
                    if new_points.contains(&Point::new(x, y)) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
        }
    }
}
