#[derive(Ord, PartialOrd, PartialEq, std::cmp::Eq)]
pub struct Crab {
    pub position: i32,
}

impl Crab {
    pub fn new(position: i32) -> Self {
        Crab { position }
    }

    pub fn compute_movement_cost(&self, new_position: i32) -> i32 {
        (new_position - self.position).abs()
    }

    pub fn compute_nonconstant_movement_cost(&self, new_position: i32) -> i32 {
        let n = (new_position - self.position).abs();
        (n * (n + 1)) / 2
    }
}
