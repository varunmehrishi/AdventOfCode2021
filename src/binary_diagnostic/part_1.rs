pub fn get_gamma_epsilon<const T: usize>(counts: &[u32; T], majority_point: u32) -> (u32, u32) {
    let mut gamma_bin = String::new();
    let mut epsilon_bin = String::new();

    for count in counts {
        let is_one_in_majority = count >= &majority_point;

        gamma_bin += &((is_one_in_majority as u32).to_string());
        epsilon_bin += &((!is_one_in_majority as u32).to_string());
    }

    let gamma = usize::from_str_radix(&gamma_bin, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_bin, 2).unwrap();

    (gamma as u32, epsilon as u32)
}
