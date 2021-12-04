#[derive(Debug, Clone)]
pub(super) struct Board<const N: usize> {
    board: [[(u32, bool); N]; N],
}

impl<const N: usize> Board<N> {
    pub fn new(values: &[[u32; N]; N]) -> Board<N> {
        let mut board = [[(0, false); N]; N];
        for i in 0..N {
            for j in 0..N {
                board[i][j].0 = values[i][j];
            }
        }

        Board { board }
    }

    fn count_marked_in_row(&self, row: usize) -> usize {
        self.board[row].iter().filter(|(_, marked)| *marked).count()
    }

    fn count_marked_in_column(&self, col: usize) -> usize {
        (0..N)
            .map(|row| self.board[row][col])
            .filter(|(_, marked)| *marked)
            .count()
    }

    pub fn won(&self) -> bool {
        (0..N)
            .map(|row| self.count_marked_in_row(row))
            .any(|marked| marked == N)
            || (0..N)
                .map(|col| self.count_marked_in_column(col))
                .any(|marked| marked == N)
    }

    pub fn mark(&mut self, value: u32) -> u32 {
        let mut marked = 0;
        for i in 0..N {
            for j in 0..N {
                if self.board[i][j].0 == value {
                    self.board[i][j].1 = true;
                    marked += 1;
                }
            }
        }
        marked
    }

    pub fn count_total_unmarked_value(&self) -> u32 {
        self.board
            .iter()
            .map(|row| -> u32 {
                row.iter()
                    .filter(|(_, marked)| !*marked)
                    .map(|(val, _)| val)
                    .sum()
            })
            .sum()
    }
}
