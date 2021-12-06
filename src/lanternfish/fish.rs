#[derive(Debug, Hash, Clone, PartialEq, std::cmp::Eq)]
pub struct Fish {
    pub timer: i32,
}

impl Fish {
    const RESET_TIME: i32 = 6;
    const NEW_FISH_DELAY: i32 = 2;
    pub fn new(timer: i32) -> Self {
        Fish { timer }
    }

    pub fn age(&mut self) -> Option<Fish> {
        match self.timer {
            0 => {
                self.timer = Fish::RESET_TIME;
                Some(Fish::new(Fish::RESET_TIME + Fish::NEW_FISH_DELAY))
            }
            _ => {
                self.timer -= 1;
                None
            }
        }
    }
}
