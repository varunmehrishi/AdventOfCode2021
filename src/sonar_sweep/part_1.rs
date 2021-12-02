pub fn find_increments(values: &[i32]) -> i32 {
    values
        .windows(2)
        .map(|w: &[i32]| (w[1] > w[0]) as i32)
        .sum()
}
