use rand::prelude::SliceRandom;
use rand::Rng;

const SQUARE_SIZE: usize = 3;

pub fn generate_board(size: usize, difficulty: usize) -> Vec<Vec<usize>> {
    loop {
        let mut board = vec![vec![0; size]; size];
        let mut rng = rand::thread_rng();

        // Fill the diagonal blocks
        for i in (0..size).step_by((size as f64).sqrt() as usize) {
            fill_block(&mut board, i, i, &mut rng);
        }

        // Solve the board
        if !resolv_backtrack(&mut board, 0, 0) {
            continue;
        }

        // Remove numbers while maintaining a unique solution
        remove_num(&mut board, difficulty, &mut rng);
        return board;
    }
}

// Fill a square block with random numbers
fn fill_block(board: &mut [Vec<usize>], row: usize, col: usize, rng: &mut impl Rng) {
    let mut nums: Vec<usize> = (1..=board.len()).collect();
    nums.shuffle(rng);

    for i in 0..SQUARE_SIZE {
        for j in 0..SQUARE_SIZE {
            board[row + i][col + j] = nums[i * SQUARE_SIZE + j];
        }
    }
}

fn remove_num(board: &mut [Vec<usize>], difficulty: usize, rng: &mut impl Rng) -> bool {
    let size = board.len();
    let to_remove = match difficulty {
        1 => (size * size) / 2,
        2 => (size * size * 3) / 5,
        3 => (size * size * 7) / 10,
        4 => size * size - 17,
        _ => (size * size) / 2,
    };

    let mut positions: Vec<(usize, usize)> = (0..size)
        .flat_map(|r| (0..size).map(move |c| (r, c)))
        .collect();
    positions.shuffle(rng);

    for _ in 0..to_remove {
        if let Some((row, col)) = positions.pop() {
            board[row][col] = 0;
        }
    }

    true
}

// Check if a number is valid in a cell (row, col)
pub fn is_num_valid(board: &[Vec<usize>], row: usize, col: usize, num: usize) -> bool {
    let size = board.len();

    if (0..size).any(|i| board[row][i] == num || board[i][col] == num) {
        return false;
    }

    let sub_row = (row / SQUARE_SIZE) * SQUARE_SIZE;
    let sub_col = (col / SQUARE_SIZE) * SQUARE_SIZE;

    board.iter().skip(sub_row).take(SQUARE_SIZE).any(|row| {
        row.iter()
            .skip(sub_col)
            .take(SQUARE_SIZE)
            .any(|&cell| cell == num)
    });

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
