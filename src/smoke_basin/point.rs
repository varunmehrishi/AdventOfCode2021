#[derive(Debug, Copy, Clone, PartialEq, Hash, std::cmp::Eq)]
pub struct Point {
    pub value: u32,
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(value: u32, x: usize, y: usize) -> Self {
        Point { value, x, y }
    }

    pub fn is_low_point<const M: usize>(&self, floor: &[[Point; M]]) -> bool {
        let mut is_lower = true;
        for neighbor in self.get_neighbors(floor) {
            is_lower &= self.is_lower_than(neighbor)
        }

        is_lower
    }

    pub fn is_lower_than(&self, other: &Self) -> bool {
        self.value < other.value
    }

    pub fn get_neighbors<'a, const M: usize>(&self, floor: &'a [[Point; M]]) -> Vec<&'a Point> {
        let n = floor.len() as i32;
        let m = M as i32;

        let mut neighbors = vec![];
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = (self.x as i32) + dx;
            let y = (self.y as i32) + dy;

            if (x >= 0 && x < n) && (y >= 0 && y < m) {
                neighbors.push(&floor[x as usize][y as usize]);
            }
        }
        neighbors
    }
}
