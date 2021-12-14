use core::panic;
use std::path::Path;

mod input;
mod part_1;
mod part_2;

#[allow(unstable_name_collisions)]
pub fn solve(file_path: &Path) {
    let (template, rules) = input::read_template_and_rules(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let answer = part_1::find_answer(&template, &rules);
    println!("Extended Polymerization Part 1 Answer: {}", answer);

    let answer = part_2::find_answer(&template, &rules);
    println!("Extended Polymerization Part 2 Answer: {}", answer);
}
