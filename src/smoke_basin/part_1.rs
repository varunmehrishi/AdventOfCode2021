use super::point::Point;

pub fn get_low_point_score<const N: usize, const M: usize>(points: [[Point; M]; N]) -> u32 {
    points
        .iter()
        .map(|row| -> u32 {
            row.iter()
                .filter(|p| p.is_low_point(&points))
                .map(|p| p.value + 1)
                .sum()
        })
        .sum()
}
