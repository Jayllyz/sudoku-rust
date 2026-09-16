use fastrand;

const SQUARE_SIZE: usize = 3;

pub fn generate_board(size: usize, difficulty: usize) -> Vec<Vec<usize>> {
    // Bounded retry: a correctly working generator succeeds almost immediately.
    const MAX_ATTEMPTS: usize = 128;

    for _ in 0..MAX_ATTEMPTS {
        let mut board = vec![vec![0; size]; size];

        // Fill the diagonal blocks
        for i in (0..size).step_by(SQUARE_SIZE) {
            fill_block(&mut board, i, i);
        }

        // Solve the board
        if !resolv_backtrack(&mut board, 0, 0) {
            continue;
        }

        // Remove numbers
        remove_numbers(&mut board, difficulty);
        return board;
    }

    panic!("failed to generate a solvable board after {MAX_ATTEMPTS} attempts");
}

// Fill a square block with random numbers
fn fill_block(board: &mut [Vec<usize>], row: usize, col: usize) {
    let mut nums: Vec<usize> = (1..=board.len()).collect();
    fastrand::shuffle(&mut nums);

    for i in 0..SQUARE_SIZE {
        for j in 0..SQUARE_SIZE {
            board[row + i][col + j] = nums[i * SQUARE_SIZE + j];
        }
    }
}

fn remove_numbers(board: &mut [Vec<usize>], difficulty: usize) {
    let size = board.len();
    let total_cells = size * size;
    let to_remove = match difficulty {
        2 => total_cells * 4 / 9, // Medium: remove 4/9
        3 => total_cells * 2 / 3, // Very Hard: remove 2/3
        _ => total_cells / 3,     // Easy: remove 1/3
    };

    let mut positions: Vec<(usize, usize)> =
        (0..size).flat_map(|r| (0..size).map(move |c| (r, c))).collect();
    fastrand::shuffle(&mut positions);

    for (row, col) in positions.iter().take(to_remove) {
        board[*row][*col] = 0;
    }
}

// Check if a number is valid in a cell (row, col)
pub fn is_num_valid(board: &[Vec<usize>], row: usize, col: usize, num: usize) -> bool {
    let size = board.len();

    if (0..size).any(|i| board[row][i] == num || board[i][col] == num) {
        return false;
    }

    let sub_row = (row / SQUARE_SIZE) * SQUARE_SIZE;
    let sub_col = (col / SQUARE_SIZE) * SQUARE_SIZE;

    if board
        .iter()
        .skip(sub_row)
        .take(SQUARE_SIZE)
        .any(|row| row.iter().skip(sub_col).take(SQUARE_SIZE).any(|&cell| cell == num))
    {
        return false;
    }

    true
}

// backtracking algorithm
// https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking
// inspired by https://gist.github.com/raeffu/8331328

pub fn resolv_backtrack(board: &mut [Vec<usize>], mut row: usize, mut col: usize) -> bool {
    if col == board.len() {
        col = 0;
        row += 1;
        if row == board.len() {
            // end of board
            return true;
        }
    }

    if board[row][col] != 0 {
        return resolv_backtrack(board, row, col + 1);
    }

    for num in 1..=board.len() {
        if is_num_valid(board, row, col, num) {
            board[row][col] = num;
            if resolv_backtrack(board, row, col + 1) {
                // found a number
                return true;
            }
            // backtrack
            board[row][col] = 0;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_board() {
        let board = generate_board(9, 1);
        assert_eq!(board.len(), 9);
        assert_eq!(board[0].len(), 9);
    }

    #[test]
    fn test_fill_block() {
        let mut board = vec![vec![0; 9]; 9];
        fill_block(&mut board, 0, 0);

        let mut numbers = [0; 9];
        for (i, row) in board.iter().take(3).enumerate() {
            for (j, &cell) in row.iter().take(3).enumerate() {
                numbers[i * 3 + j] = cell;
            }
        }
        numbers.sort_unstable();
        assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_fill_block_at_offset() {
        let mut board = vec![vec![0; 9]; 9];
        fill_block(&mut board, 3, 6);

        let mut numbers = Vec::new();
        for row in board.iter().skip(3).take(3) {
            for &cell in row.iter().skip(6).take(3) {
                numbers.push(cell);
            }
        }
        numbers.sort_unstable();
        assert_eq!(numbers, (1..=9).collect::<Vec<_>>());

        // Everything outside the requested block must be left untouched.
        for (r, row) in board.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if !(3..6).contains(&r) || !(6..9).contains(&c) {
                    assert_eq!(cell, 0, "cell ({r},{c}) should be untouched");
                }
            }
        }
    }

    #[test]
    fn test_remove_numbers() {
        let mut board = vec![vec![1; 9]; 9];
        remove_numbers(&mut board, 1);

        let zeros = board.iter().flatten().filter(|&&x| x == 0).count();
        assert!(zeros > 0);
    }

    #[test]
    fn test_remove_numbers_exact_count_by_difficulty() {
        let size = 9;
        let total_cells = size * size;

        // Difficulty 1 is the default, so unknown values fall back to the easy count.
        let cases = [
            (1, total_cells / 3),
            (2, total_cells * 4 / 9),
            (3, total_cells * 2 / 3),
            (0, total_cells / 3),
            (4, total_cells / 3),
        ];

        for (difficulty, expected) in cases {
            let mut board = vec![vec![7; size]; size];
            remove_numbers(&mut board, difficulty);

            let zeros = board.iter().flatten().filter(|&&x| x == 0).count();
            assert_eq!(zeros, expected, "difficulty {difficulty} removed the wrong amount");
        }
    }

    #[test]
    fn test_remove_numbers_scales_with_board_size() {
        let size = 9;
        let mut small = vec![vec![7; size]; size];
        let mut large = vec![vec![7; size * 2]; size * 2];

        remove_numbers(&mut small, 1);
        remove_numbers(&mut large, 1);

        let small_zeros = small.iter().flatten().filter(|&&x| x == 0).count();
        let large_zeros = large.iter().flatten().filter(|&&x| x == 0).count();

        assert!(large_zeros > small_zeros);
    }

    #[test]
    fn test_is_num_valid() {
        let board = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        assert!(is_num_valid(&board, 0, 2, 4));
        assert!(!is_num_valid(&board, 0, 2, 3));
    }

    #[test]
    fn test_is_num_valid_matches_reference() {
        let size = 9;
        let num = 7;

        for conflict_row in 0..size {
            for conflict_col in 0..size {
                let mut board = vec![vec![0; size]; size];
                board[conflict_row][conflict_col] = num;

                for query_row in 0..size {
                    for query_col in 0..size {
                        if (query_row, query_col) == (conflict_row, conflict_col) {
                            continue;
                        }

                        let conflicts = conflict_row == query_row
                            || conflict_col == query_col
                            || (conflict_row / SQUARE_SIZE == query_row / SQUARE_SIZE
                                && conflict_col / SQUARE_SIZE == query_col / SQUARE_SIZE);

                        assert_eq!(
                            is_num_valid(&board, query_row, query_col, num),
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
        let mut board = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        assert!(resolv_backtrack(&mut board, 0, 0));

        for row in &board {
            assert!(row.iter().all(|&x| x != 0));
        }
    }

    #[test]
    fn test_generate_board_is_solvable() {
        for difficulty in 1..=3 {
            let board = generate_board(9, difficulty);

            assert_eq!(board.len(), 9);
            assert!(board.iter().all(|row| row.len() == 9));

            let mut solution = board.clone();
            assert!(resolv_backtrack(&mut solution, 0, 0), "difficulty {difficulty}");
            assert!(solution.iter().flatten().all(|&x| x != 0));
        }
    }

    #[test]
    fn test_generate_board_different_difficulties() {
        let easy_board = generate_board(9, 1);
        let medium_board = generate_board(9, 2);
        let hard_board = generate_board(9, 3);

        let count_zeros =
            |board: &Vec<Vec<usize>>| board.iter().flatten().filter(|&&x| x == 0).count();

        let easy_zeros = count_zeros(&easy_board);
        let medium_zeros = count_zeros(&medium_board);
        let hard_zeros = count_zeros(&hard_board);

        assert!(easy_zeros < medium_zeros);
        assert!(medium_zeros < hard_zeros);
    }
}
