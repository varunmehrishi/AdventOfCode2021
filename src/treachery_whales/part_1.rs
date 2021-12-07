use super::crab::Crab;

fn compute_movement_cost(crabs: &[Crab], new_position: i32) -> i32 {
    crabs
        .iter()
        .map(|crab| crab.compute_movement_cost(new_position))
        .sum()
}

pub fn compute_min_cost(crabs: &[Crab]) -> i32 {
    let min_position = crabs.iter().min().unwrap().position;
    let max_position = crabs.iter().max().unwrap().position;

    (min_position..=max_position)
        .map(|position| compute_movement_cost(crabs, position))
        .min()
        .unwrap()
}
