use super::point::Point;
use std::collections::VecDeque;

pub fn get_largest_basins_score<const N: usize, const M: usize>(matrix: [[Point; M]; N]) -> i64 {
    let mut values = vec![];

    let mut visited = [[false; M]; N];
    for row in matrix {
        for point in row {
            let mut q = VecDeque::new();
            q.push_back(point);
            let mut count = 0;
            while let Some(p) = q.pop_front() {
                if p.value != 9 && !visited[p.x][p.y] {
                    for n in p.get_neighbors(&matrix) {
                        q.push_back(*n);
                    }
                    visited[p.x][p.y] = true;
                    count += 1;
                }
            }
            values.push(count as i64);
        }
    }

    values.sort_unstable();
    values.iter().rev().take(3).product()
}
