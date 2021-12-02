pub fn find_increments(values: &[i32]) -> i32 {
    let windows: Vec<_> = values.windows(3).map(|x| x.iter().sum::<i32>()).collect();

    let increments: i32 = windows
        .windows(2)
        .map(|w: &[i32]| (w[1] > w[0]) as i32)
        .sum();

    increments
}
