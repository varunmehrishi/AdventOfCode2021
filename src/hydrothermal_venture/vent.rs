pub struct Vent {
    pub start: (i32, i32),
    pub stop: (i32, i32),
}

impl Vent {
    pub fn new(start: (i32, i32), stop: (i32, i32)) -> Self {
        if start.0 > stop.0 {
            Vent {
                start: stop,
                stop: start,
            }
        } else {
            Vent { start, stop }
        }
    }

    pub fn is_vertical(&self) -> bool {
        self.start.0 == self.stop.0
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.stop.1
    }

    pub fn is_diagonal(&self) -> bool {
        self.delta_x().abs() == self.delta_y().abs()
    }

    pub fn delta_x(&self) -> i32 {
        self.stop.0 - self.start.0
    }

    pub fn delta_y(&self) -> i32 {
        self.stop.1 - self.start.1
    }
}
