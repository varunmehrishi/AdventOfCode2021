use super::floor::Floor;
use super::vent::Vent;

pub fn get_intersection_count(vents: &[Vent]) -> i32 {
    let mut floor: Floor<1000> = Floor::new();

    vents
        .iter()
        .filter(|x| x.is_vertical())
        .for_each(|vent| floor.mark_vertical(vent));

    vents
        .iter()
        .filter(|x| x.is_horizontal())
        .for_each(|vent| floor.mark_horizontal(vent));

    vents
        .iter()
        .filter(|x| x.is_diagonal())
        .for_each(|vent| floor.mark_diagonal(vent));

    floor.count_marked_greater_than(1)
}
