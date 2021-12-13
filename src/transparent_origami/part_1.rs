use super::{fold, FoldAlong, Point};
use std::collections::HashSet;

pub(super) fn count_new_points(points: &HashSet<Point>, fold_op: &FoldAlong) -> usize {
    fold(points, fold_op).len()
}
