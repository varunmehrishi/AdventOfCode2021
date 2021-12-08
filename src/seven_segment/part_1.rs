use std::collections::HashSet;

pub fn count_uniquely_identifiable(values: &[([String; 10], [String; 4])]) -> usize {
    let set: HashSet<usize> = [2usize, 3, 4, 7].iter().copied().collect();

    values
        .iter()
        .map(|(_, right)| right.iter().filter(|x| set.contains(&x.len())).count())
        .sum::<usize>()
}
