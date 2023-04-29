use rand::Rng;
use std::io;
use std::process;

const BOARD_SIZE: usize = 9;
const SQUARE_SIZE: usize = 3;

#[cfg(test)]
mod tests {
    use crate::{generate, print_board};

    #[test]
    fn board_valid() {
        const BOARD_SIZE: usize = 9;
        let board = generate(BOARD_SIZE);
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
    }
}

fn main() {
    println!("Welcome to Sudoku-rust");
    let mut board;

    loop {
        println!("1. Generate");
        println!("2. Solve (coming later)");
        println!("3. Exit");
        println!("Enter your choice: ");
        let choice = read_int();
        match choice {
            1 => {
                println!();
                board = generate(BOARD_SIZE);
                print_board(&board);
            }
            2 => println!("Coming later..."),
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

fn generate(size: usize) -> Vec<Vec<usize>> {
    let mut board = vec![vec![0; size]; size];
    let mut rng = rand::thread_rng();
    for row in 0..size {
        for col in 0..size {
            let num: usize = rng.gen_range(0..=BOARD_SIZE);
            if num == 0 {
                board[row][col] = 0;
                continue;
            } else if is_num_valid(&board, row, col, num) {
                board[row][col] = num;
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
