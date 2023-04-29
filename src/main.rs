use rand::Rng;
use std::io;
use std::process;

const BOARD_SIZE: usize = 9;
const SQUARE_SIZE: usize = 3;

#[cfg(test)]
mod tests {
    use crate::{generate, print_board, resolv_backtrack};

    #[test]
    fn board_valid() {
        const BOARD_SIZE: usize = 9;
        let board = generate(BOARD_SIZE, 1);
        print_board(&board);
        assert_eq!(board.len(), 9);

        let mut hm = std::collections::HashMap::new();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if hm.contains_key(&board[i][j]) {
                    assert!(false);
                }
                if board[i][j] != 0 {
                    hm.insert(board[i][j], true);
                }
            }
            hm.clear();
        }
        assert_eq!(resolv_backtrack(&mut board.clone(), 0, 0), true);
    }
}

fn main() {
    println!("Welcome to Sudoku-rust");
    let mut board = vec![];

    loop {
        println!("1. Generate sudoku (9x9)");
        println!("2. Solve sudoku (backtracking)");
        println!("3. Exit");
        println!("Enter your choice: ");
        let choice = read_int();
        match choice {
            1 => {
                println!();
                println!("1. Easy");
                println!("2. Medium");
                println!("3. Hard");
                println!("Enter your choice: ");

                let difficulty = read_difficulty();

                board = generate(BOARD_SIZE, difficulty);
                print_board(&board);
            }
            2 => {
                if board.len() == 0 {
                    println!("Please generate a board first");
                    continue;
                }

                if resolv_backtrack(&mut board, 0, 0) {
                    println!();
                    print_board(&board);
                } else {
                    // should never happen
                    println!("No solution found");
                }
            }
            3 => process::exit(0),
            _ => println!("Invalid choice"),
        }
    }
}

fn read_int() -> usize {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input | please enter a number"),
        }
    }
}

fn read_difficulty() -> usize {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num > 0 && num <= 3 {
                    return num;
                } else {
                    println!("Invalid input | please enter a number between 1 and 3");
                }
            }
            Err(_) => println!("Invalid input | please enter a number"),
        }
    }
}

fn generate(size: usize, difficulty: usize) -> Vec<Vec<usize>> {
    let mut board = vec![vec![0; size]; size];
    let mut rng = rand::thread_rng();
    let luck: f64;
    match difficulty {
        1 => luck = 0.4,
        2 => luck = 0.45,
        3 => luck = 0.5,
        _ => luck = 0.4,
    }
    resolv_backtrack(&mut board, 0, 0); // generate a valid board
    for i in 0..size {
        for j in 0..size {
            if rng.gen_bool(luck) {
                board[i][j] = 0;
            }
        }
    }
    board
}

fn is_num_valid(board: &Vec<Vec<usize>>, row: usize, col: usize, num: usize) -> bool {
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

fn print_board(board: &Vec<Vec<usize>>) {
    for i in 0..board.len() {
        for j in 0..board.len() {
            if board[i][j] == 0 {
                print!(" . ");
            } else {
                print!(" {} ", board[i][j]);
            }
            if (j + 1) % SQUARE_SIZE == 0 {
                print!(" ");
            }
        }
        println!();
        if (i + 1) % SQUARE_SIZE == 0 {
            println!();
        }
    }
}

// backtracking algorithm
// https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking
// inspired by https://gist.github.com/raeffu/8331328

fn resolv_backtrack(board: &mut Vec<Vec<usize>>, mut row: usize, mut col: usize) -> bool {
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
