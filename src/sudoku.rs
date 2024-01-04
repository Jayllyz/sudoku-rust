use rand::Rng;

const SQUARE_SIZE: usize = 3;

pub fn generate(size: usize, difficulty: usize) -> Vec<Vec<usize>> {
    let mut board = vec![vec![0; size]; size];
    let mut rng = rand::thread_rng();
    let luck: f64 = match difficulty {
        1 => 0.4,
        2 => 0.5,
        3 => 0.6,
        _ => 0.4,
    };

    resolv_backtrack(&mut board, 0, 0); // generate a valid board
    for _ in 0..size {
        for j in 0..size {
            if rng.gen_bool(luck) {
                board[j][j] = 0;
            }
        }
    }
    board
}
pub fn is_num_valid(board: &Vec<Vec<usize>>, row: usize, col: usize, num: usize) -> bool {
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
