use rand::prelude::SliceRandom;
use rand::Rng;

const SQUARE_SIZE: usize = 3;

pub fn generate_board(size: usize, difficulty: usize) -> Vec<Vec<usize>> {
    let mut board = vec![vec![0; size]; size];
    let mut rng = rand::thread_rng();

    // Fill the diagonal blocks, this is the "seed"
    for i in (0..size).step_by((size as f64).sqrt() as usize) {
        fill_block(&mut board, i, i, &mut rng);
    }

    // Solve the board
    let res = resolv_backtrack(&mut board, 0, 0);
    if !res {
        return generate_board(size, difficulty);
    }

    let keep: usize = match difficulty {
        // Easy keep 50% of the numbers
        1 => ((board.len() as f64 * board.len() as f64) * 0.5) as usize,
        // Medium keep 40% of the numbers
        2 => ((board.len() as f64 * board.len() as f64) * 0.4) as usize,
        // Hard keep 30% of the numbers
        3 => ((board.len() as f64 * board.len() as f64) * 0.3) as usize,
        // Maximum difficulty keep 17 numbers
        4 => 17,
        _ => ((board.len() as f64 * board.len() as f64) * 0.5) as usize,
    };
    let mut counter = board.len() as usize * board.len() as usize;

    while counter > keep {
        let row = rng.gen_range(0..size);
        let col = rng.gen_range(0..size);
        if board[row][col] != 0 {
            board[row][col] = 0;
            counter -= 1;
        }
    }

    board
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

// Check if a number is valid in a cell (row, col)
pub fn is_num_valid(board: &[Vec<usize>], row: usize, col: usize, num: usize) -> bool {
    for i in 0..board.len() {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }

    let sub_row = (row / SQUARE_SIZE) * SQUARE_SIZE;
    let sub_col = (col / SQUARE_SIZE) * SQUARE_SIZE;
    for i in 0..SQUARE_SIZE {
        for j in 0..SQUARE_SIZE {
            if board[sub_row + i][sub_col + j] == num {
                return false;
            }
        }
    }
    true
}

// backtracking algorithm
// https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking
// inspired by https://gist.github.com/raeffu/8331328

pub fn resolv_backtrack(board: &mut Vec<Vec<usize>>, mut row: usize, mut col: usize) -> bool {
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
