use super::Board;

pub(super) fn get_last_winning_board_score<const N: usize>(
    values: &[u32],
    mut boards: Vec<Board<N>>,
) -> u32 {
    let mut winning_boards = vec![];

    for value in values {
        boards.iter_mut().for_each(|board| {
            board.mark(*value);
            if board.won() {
                winning_boards.push((board.clone(), value));
            }
        });

        boards = boards
            .iter()
            .filter(|board| !board.won())
            .cloned()
            .collect();
    }

    let (board, value) = &winning_boards[winning_boards.len() - 1];

    *value * board.count_total_unmarked_value()
}
