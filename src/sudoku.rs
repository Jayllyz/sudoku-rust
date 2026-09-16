use fastrand;

// Digits are tracked as bits in a `u32`, so the largest board we can represent
// is the largest perfect square that fits in 32 bits: 25 (a 5x5 grid of blocks).
const MAX_SIZE: usize = 25;

/// A square sudoku board.
///
/// The side length must be a perfect square (`4`, `9`, `16`, `25`) so that the
/// board divides into equally sized square blocks.
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    size: usize,
    block: usize,
    cells: Vec<Vec<usize>>,
}

impl Board {
    /// Create an empty `size x size` board.
    ///
    /// # Panics
    ///
    /// Panics if `size` is not a perfect square in `1..=MAX_SIZE`.
    pub fn new(size: usize) -> Self {
        let block = size.isqrt();
        assert!(block * block == size, "board size {size} is not a perfect square");
        assert!(size <= MAX_SIZE, "board size {size} exceeds the maximum of {MAX_SIZE}");

        Self { size, block, cells: vec![vec![0; size]; size] }
    }

    /// Wrap existing cells, validating the shape and the values.
    ///
    /// # Panics
    ///
    /// Panics if `cells` is not a square of a supported size, or holds a value
    /// larger than the board size.
    pub fn from_rows(cells: Vec<Vec<usize>>) -> Self {
        let size = cells.len();
        let block = size.isqrt();
        assert!(block * block == size, "board size {size} is not a perfect square");
        assert!(size <= MAX_SIZE, "board size {size} exceeds the maximum of {MAX_SIZE}");
        assert!(cells.iter().all(|row| row.len() == size), "board must be square");
        assert!(cells.iter().flatten().all(|&value| value <= size), "cell value out of range");

        Self { size, block, cells }
    }

    /// Generate a random, solvable board of the given difficulty.
    ///
    /// # Panics
    ///
    /// Panics if `size` is not a supported board size, or if generation keeps
    /// failing (which should not happen for a correct generator).
    pub fn generate(size: usize, difficulty: usize) -> Self {
        // Bounded retry: a correctly working generator succeeds almost immediately.
        const MAX_ATTEMPTS: usize = 128;

        for _ in 0..MAX_ATTEMPTS {
            let mut board = Self::new(size);
            let block = board.block;

            // Fill the diagonal blocks; the solver completes the rest.
            for i in (0..size).step_by(block) {
                board.fill_block(i, i);
            }

            if !board.resolv_backtrack() {
                continue;
            }

            board.remove_numbers(difficulty);
            return board;
        }

        panic!("failed to generate a solvable board after {MAX_ATTEMPTS} attempts");
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn block(&self) -> usize {
        self.block
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> usize {
        self.cells[row][col]
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: usize) {
        self.cells[row][col] = value;
    }

    /// The board as rows, ready to be rendered.
    pub fn rows(&self) -> &[Vec<usize>] {
        &self.cells
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().flatten().all(|&value| value != 0)
    }

    /// Check if `num` may be placed at `(row, col)`.
    #[inline]
    pub fn is_num_valid(&self, row: usize, col: usize, num: usize) -> bool {
        // Dispatch on the block size so the `/` and `%` fold to constants.
        match self.block {
            1 => self.is_num_valid_in_block::<1>(row, col, num),
            2 => self.is_num_valid_in_block::<2>(row, col, num),
            3 => self.is_num_valid_in_block::<3>(row, col, num),
            4 => self.is_num_valid_in_block::<4>(row, col, num),
            5 => self.is_num_valid_in_block::<5>(row, col, num),
            _ => unreachable!("unsupported block size {}", self.block),
        }
    }

    #[inline]
    fn is_num_valid_in_block<const B: usize>(&self, row: usize, col: usize, num: usize) -> bool {
        let size = self.size;
        let cells = &self.cells;

        let sub_row = (row / B) * B;
        let sub_col = (col / B) * B;

        let line_conflict = (0..size).any(|i| cells[row][i] == num || cells[i][col] == num);
        let box_conflict = cells[sub_row..sub_row + B]
            .iter()
            .any(|line| line[sub_col..sub_col + B].contains(&num));

        !(line_conflict || box_conflict)
    }

    /// Solve the board in place, returning whether a solution was found.
    ///
    /// Uses backtracking; see
    /// <https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking>.
    pub fn resolv_backtrack(&mut self) -> bool {
        // Dispatch to a solver monomorphized for the board's size so `size` and
        // `block` are compile-time constants in the hot path.
        match (self.size, self.block) {
            (1, 1) => self.run_solver::<1, 1>(),
            (4, 2) => self.run_solver::<4, 2>(),
            (9, 3) => self.run_solver::<9, 3>(),
            (16, 4) => self.run_solver::<16, 4>(),
            (25, 5) => self.run_solver::<25, 5>(),
            _ => unreachable!("unsupported board size {}", self.size),
        }
    }

    fn run_solver<const N: usize, const B: usize>(&mut self) -> bool {
        let mut state = BoardState::<N, B>::from_board(self);
        solve::<N, B>(&mut state, self, 0, 0)
    }

    /// Fill a block with a shuffled copy of every digit.
    fn fill_block(&mut self, row: usize, col: usize) {
        let block = self.block;
        let mut numbers: Vec<usize> = (1..=self.size).collect();
        fastrand::shuffle(&mut numbers);

        for i in 0..block {
            for j in 0..block {
                self.cells[row + i][col + j] = numbers[i * block + j];
            }
        }
    }

    /// Remove a difficulty-dependent number of cells.
    fn remove_numbers(&mut self, difficulty: usize) {
        let total_cells = self.size * self.size;
        let to_remove = match difficulty {
            2 => total_cells * 4 / 9, // Medium: remove 4/9
            3 => total_cells * 2 / 3, // Very Hard: remove 2/3
            _ => total_cells / 3,     // Easy: remove 1/3
        };

        let mut positions: Vec<(usize, usize)> =
            (0..self.size).flat_map(|r| (0..self.size).map(move |c| (r, c))).collect();
        fastrand::shuffle(&mut positions);

        for (row, col) in positions.iter().take(to_remove) {
            self.cells[*row][*col] = 0;
        }
    }
}

// Tracks which digits already appear in every row, column and box using one
// bitmask per unit, turning conflict checks into a few bitwise operations.
//
// `N` is the board side length and `B` the block side length; both are const
// generics so the compiler can fold every index and bound computation.
struct BoardState<const N: usize, const B: usize> {
    rows: [u32; N],
    cols: [u32; N],
    boxes: [u32; N],
}

impl<const N: usize, const B: usize> BoardState<N, B> {
    fn from_board(board: &Board) -> Self {
        let mut state = Self { rows: [0; N], cols: [0; N], boxes: [0; N] };

        for row in 0..N {
            for col in 0..N {
                let value = board.get(row, col);
                if value != 0 {
                    state.set(row, col, value, true);
                }
            }
        }

        state
    }

    #[inline]
    fn box_index(row: usize, col: usize) -> usize {
        (row / B) * B + col / B
    }

    #[inline]
    fn update(mask: &mut u32, bit: u32, present: bool) {
        if present {
            *mask |= bit;
        } else {
            *mask &= !bit;
        }
    }

    #[inline]
    fn set(&mut self, row: usize, col: usize, num: usize, present: bool) {
        let bit = 1u32 << num;
        let index = Self::box_index(row, col);
        Self::update(&mut self.rows[row], bit, present);
        Self::update(&mut self.cols[col], bit, present);
        Self::update(&mut self.boxes[index], bit, present);
    }

    #[inline]
    fn can_place(&self, row: usize, col: usize, num: usize) -> bool {
        let bit = 1u32 << num;
        let index = Self::box_index(row, col);
        self.rows[row] & bit == 0 && self.cols[col] & bit == 0 && self.boxes[index] & bit == 0
    }
}

fn solve<const N: usize, const B: usize>(
    state: &mut BoardState<N, B>,
    board: &mut Board,
    mut row: usize,
    mut col: usize,
) -> bool {
    if col == N {
        col = 0;
        row += 1;
        if row == N {
            // end of board
            return true;
        }
    }

    if board.get(row, col) != 0 {
        return solve::<N, B>(state, board, row, col + 1);
    }

    for num in 1..=N {
        if state.can_place(row, col, num) {
            board.set(row, col, num);
            state.set(row, col, num, true);
            if solve::<N, B>(state, board, row, col + 1) {
                // found a number
                return true;
            }
            // backtrack
            board.set(row, col, 0);
            state.set(row, col, num, false);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_zeros(board: &Board) -> usize {
        board.rows().iter().flatten().filter(|&&value| value == 0).count()
    }

    fn filled_board(size: usize, value: usize) -> Board {
        let mut board = Board::new(size);
        for row in 0..size {
            for col in 0..size {
                board.set(row, col, value);
            }
        }
        board
    }

    #[test]
    fn test_new_board_is_empty_and_square() {
        let board = Board::new(9);
        assert_eq!(board.size(), 9);
        assert_eq!(board.block(), 3);
        assert!(board.rows().iter().all(|row| row.len() == 9));
        assert_eq!(count_zeros(&board), 81);
    }

    #[test]
    fn test_new_board_supported_sizes() {
        for (size, block) in [(4, 2), (9, 3), (16, 4), (25, 5)] {
            let board = Board::new(size);
            assert_eq!(board.size(), size);
            assert_eq!(board.block(), block);
            assert_eq!(board.rows().len(), size);
        }
    }

    #[test]
    #[should_panic(expected = "not a perfect square")]
    fn test_new_board_rejects_non_square_size() {
        Board::new(6);
    }

    #[test]
    #[should_panic(expected = "exceeds the maximum")]
    fn test_new_board_rejects_oversized_size() {
        Board::new(36);
    }

    #[test]
    #[should_panic(expected = "board must be square")]
    fn test_from_rows_rejects_ragged_board() {
        Board::from_rows(vec![vec![0; 8]; 9]);
    }

    #[test]
    #[should_panic(expected = "cell value out of range")]
    fn test_from_rows_rejects_invalid_values() {
        let mut cells = vec![vec![0; 9]; 9];
        cells[0][0] = 10;
        Board::from_rows(cells);
    }

    #[test]
    fn test_generate_board() {
        let board = Board::generate(9, 1);
        assert_eq!(board.size(), 9);
        assert!(board.rows().iter().all(|row| row.len() == 9));
    }

    #[test]
    fn test_generate_board_4x4() {
        let board = Board::generate(4, 1);
        assert_eq!(board.size(), 4);
        assert_eq!(board.block(), 2);
        assert_eq!(board.rows().len(), 4);
    }

    #[test]
    fn test_fill_block() {
        let mut board = Board::new(9);
        board.fill_block(0, 0);

        let mut numbers: Vec<usize> =
            board.rows().iter().take(3).flat_map(|row| row.iter().take(3)).copied().collect();
        numbers.sort_unstable();
        assert_eq!(numbers, (1..=9).collect::<Vec<_>>());
    }

    #[test]
    fn test_fill_block_at_offset() {
        let mut board = Board::new(9);
        board.fill_block(3, 6);

        let mut numbers: Vec<usize> = board
            .rows()
            .iter()
            .skip(3)
            .take(3)
            .flat_map(|row| row.iter().skip(6).take(3))
            .copied()
            .collect();
        numbers.sort_unstable();
        assert_eq!(numbers, (1..=9).collect::<Vec<_>>());

        // Everything outside the requested block must be left untouched.
        for (r, row) in board.rows().iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if !(3..6).contains(&r) || !(6..9).contains(&c) {
                    assert_eq!(cell, 0, "cell ({r},{c}) should be untouched");
                }
            }
        }
    }

    #[test]
    fn test_remove_numbers() {
        let mut board = filled_board(9, 1);
        board.remove_numbers(1);

        assert!(count_zeros(&board) > 0);
    }

    #[test]
    fn test_remove_numbers_exact_count_by_difficulty() {
        let total_cells = 9 * 9;

        // Difficulty 1 is the default, so unknown values fall back to the easy count.
        let cases = [
            (1, total_cells / 3),
            (2, total_cells * 4 / 9),
            (3, total_cells * 2 / 3),
            (0, total_cells / 3),
            (4, total_cells / 3),
        ];

        for (difficulty, expected) in cases {
            let mut board = filled_board(9, 7);
            board.remove_numbers(difficulty);

            assert_eq!(
                count_zeros(&board),
                expected,
                "difficulty {difficulty} removed the wrong amount"
            );
        }
    }

    #[test]
    fn test_remove_numbers_scales_with_board_size() {
        let mut small = Board::new(4);
        let mut large = Board::new(9);

        small.remove_numbers(1);
        large.remove_numbers(1);

        assert!(count_zeros(&large) > count_zeros(&small));
    }

    #[test]
    fn test_is_num_valid() {
        let board = Board::from_rows(vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);

        assert!(board.is_num_valid(0, 2, 4));
        assert!(!board.is_num_valid(0, 2, 3));
    }

    #[test]
    fn test_is_num_valid_matches_reference() {
        let size = 9;
        let num = 7;

        for conflict_row in 0..size {
            for conflict_col in 0..size {
                let mut board = Board::new(size);
                board.set(conflict_row, conflict_col, num);
                let block = board.block();

                for query_row in 0..size {
                    for query_col in 0..size {
                        if (query_row, query_col) == (conflict_row, conflict_col) {
                            continue;
                        }

                        let conflicts = conflict_row == query_row
                            || conflict_col == query_col
                            || (conflict_row / block == query_row / block
                                && conflict_col / block == query_col / block);

                        assert_eq!(
                            board.is_num_valid(query_row, query_col, num),
                            !conflicts,
                            "query=({query_row},{query_col}) conflict=({conflict_row},{conflict_col})"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_resolv_backtrack() {
        let mut board = Board::from_rows(vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);

        assert!(board.resolv_backtrack());
        assert!(board.is_solved());
    }

    #[test]
    fn test_solve_4x4_board() {
        let mut board = Board::from_rows(vec![
            vec![1, 0, 0, 4],
            vec![0, 4, 1, 0],
            vec![0, 1, 4, 0],
            vec![4, 0, 0, 1],
        ]);

        assert!(board.resolv_backtrack());
        assert!(board.is_solved());
    }

    #[test]
    fn test_generate_board_is_solvable() {
        for difficulty in 1..=3 {
            let mut solution = Board::generate(9, difficulty);

            assert_eq!(solution.size(), 9);
            assert!(solution.rows().iter().all(|row| row.len() == 9));
            assert!(solution.resolv_backtrack(), "difficulty {difficulty}");
            assert!(solution.is_solved());
        }
    }

    #[test]
    fn test_generate_board_different_difficulties() {
        let easy_board = Board::generate(9, 1);
        let medium_board = Board::generate(9, 2);
        let hard_board = Board::generate(9, 3);

        assert!(count_zeros(&easy_board) < count_zeros(&medium_board));
        assert!(count_zeros(&medium_board) < count_zeros(&hard_board));
    }
}
