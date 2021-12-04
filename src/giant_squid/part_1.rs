use super::Board;

pub(super) fn get_winning_board_score<const N: usize>(
    values: &[u32],
    mut boards: Vec<Board<N>>,
) -> u32 {
    for value in values {
        boards.iter_mut().for_each(|board| {
            board.mark(*value);
        });

        if let Some(max) = boards
            .iter()
            .filter(|board| board.won())
            .map(|board| board.count_total_unmarked_value())
            .max()
        {
            return max * value;
        }
    }

    0
}
