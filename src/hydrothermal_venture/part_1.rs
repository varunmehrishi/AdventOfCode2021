use super::floor::Floor;
use super::vent::Vent;

pub fn get_straight_intersection_count(vents: &Vec<Vent>) -> i32 {
    let mut floor: Floor<1000> = Floor::new();

    vents
        .iter()
        .filter(|x| x.is_vertical())
        .for_each(|vent| floor.mark_vertical(vent));

    vents
        .iter()
        .filter(|x| x.is_horizontal())
        .for_each(|vent| floor.mark_horizontal(vent));

    floor.count_marked_greater_than(1)
}
