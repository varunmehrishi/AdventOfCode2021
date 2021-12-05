use super::vent::Vent;

pub struct Floor<const N: usize> {
    floor: [[i32; N]; N],
}

impl<const N: usize> Floor<N> {
    pub fn new() -> Self {
        Floor { floor: [[0; N]; N] }
    }

    pub fn mark_vertical(&mut self, vent: &Vent) -> () {
        if vent.delta_y() > 0 {
            for y in vent.start.1..=vent.stop.1 {
                self.floor[vent.start.0 as usize][y as usize] += 1;
            }
        } else {
            for y in vent.stop.1..=vent.start.1 {
                self.floor[vent.start.0 as usize][y as usize] += 1;
            }
        }
    }

    pub fn mark_horizontal(&mut self, vent: &Vent) {
        for i in vent.start.0..=vent.stop.0 {
            self.floor[i as usize][vent.start.1 as usize] += 1;
        }
    }

    pub fn mark_diagonal(&mut self, vent: &Vent) {
        if vent.delta_y() > 0 {
            for i in 0..=vent.delta_x() {
                self.floor[(vent.start.0 + i) as usize][(vent.start.1 + i) as usize] += 1;
            }
        } else {
            for i in 0..=vent.delta_x() {
                self.floor[(vent.start.0 + i) as usize][(vent.start.1 - i) as usize] += 1;
            }
        }
    }

    pub fn count_marked_greater_than(&self, n: i32) -> i32 {
        self.floor
            .iter()
            .map(|row| row.iter().map(|x| (*x > n) as i32).sum::<i32>())
            .sum()
    }
}
