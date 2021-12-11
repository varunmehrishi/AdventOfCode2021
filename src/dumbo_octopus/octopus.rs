#[derive(Debug, Copy, Clone, PartialEq, Hash, std::cmp::Eq)]
pub struct Octopus {
    pub value: u32,
    pub x: usize,
    pub y: usize,
    pub flashed: bool,
    pub flashes: u32,
}

impl Octopus {
    pub fn new(value: u32, x: usize, y: usize) -> Self {
        Octopus {
            value,
            x,
            y,
            flashed: false,
            flashes: 0,
        }
    }

    pub fn get_neighbor_coors(&self, size: (usize, usize)) -> Vec<(usize, usize)> {
        let n = size.0 as i32;
        let m = size.1 as i32;

        let mut neighbors = vec![];
        for (dx, dy) in [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            let x = (self.x as i32) + dx;
            let y = (self.y as i32) + dy;

            if (x >= 0 && x < n) && (y >= 0 && y < m) {
                neighbors.push((x as usize, y as usize));
            }
        }
        neighbors
    }

    pub fn update(&mut self, size: (usize, usize)) -> Vec<(usize, usize)> {
        self.value += 1;

        if self.value > 9 {
            self.flash(size)
        } else {
            vec![]
        }
    }

    pub fn flash(&mut self, size: (usize, usize)) -> Vec<(usize, usize)> {
        if !self.flashed {
            self.flashed = true;
            self.flashes += 1;
            self.get_neighbor_coors(size)
        } else {
            vec![]
        }
    }

    pub fn finalize(&mut self) {
        if self.flashed {
            self.value = 0;
            self.flashed = false;
        }
    }
}
