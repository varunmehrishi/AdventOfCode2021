use super::get_count_at_index;
use std::cmp::Ordering;

pub fn get_oxygen_co2_rating<const T: usize>(values: &[[u32; T]]) -> (u32, u32) {
    let mut oxygen = values.to_vec();
    for i in 0..T {
        if oxygen.len() == 1 {
            break;
        }

        let n = oxygen.len() as u32;
        let ones = get_count_at_index(&oxygen, i);
        let zeros = n - ones;

        oxygen = match ones.cmp(&zeros) {
            Ordering::Greater => oxygen.into_iter().filter(|value| value[i] == 1).collect(),
            Ordering::Equal => oxygen.into_iter().filter(|value| value[i] == 1).collect(),
            Ordering::Less => oxygen.into_iter().filter(|value| value[i] == 0).collect(),
        }
    }

    // println!("Oxygen {:?}", oxygen);

    let mut co2 = values.to_vec();
    for i in 0..T {
        if co2.len() == 1 {
            break;
        }

        let n = co2.len() as u32;
        let ones = get_count_at_index(&co2, i);
        let zeros = n - ones;

        co2 = match ones.cmp(&zeros) {
            Ordering::Greater => co2.into_iter().filter(|value| value[i] == 0).collect(),
            Ordering::Equal => co2.into_iter().filter(|value| value[i] == 0).collect(),
            Ordering::Less => co2.into_iter().filter(|value| value[i] == 1).collect(),
        }
    }

    // println!("Co2 {:?}", co2);

    let mut oxygen_bin = String::new();
    let mut co2_bin = String::new();

    for o in &oxygen[0] {
        oxygen_bin += &o.to_string();
    }

    for c in &co2[0] {
        co2_bin += &c.to_string();
    }

    let oxygen = usize::from_str_radix(&oxygen_bin, 2).unwrap();
    let co2 = usize::from_str_radix(&co2_bin, 2).unwrap();
    (oxygen as u32, co2 as u32)
}
