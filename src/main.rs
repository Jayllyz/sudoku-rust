use rand::Rng;
use std::io;
use std::process;

const BOARD_SIZE: usize = 9;

fn main() {
    println!("Welcome to Sudoku-rust");

    let mut choice = 0;
    while choice != 3 {
        println!("1. Generate");
        println!("2. Solve (coming later)");
        println!("3. Exit");
        println!("Enter your choice: ");
        choice = read_int();
        match choice {
            1 => {
                let board = generate();
                print_board(board);
            }
            2 => println!("Coming later..."),
            3 => process::exit(0),
            _ => println!("Invalid choice"),
        }
    }
}

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    if input.trim().parse::<i32>().is_err() {
        print!("Please enter a valid integer: ");
        return read_int();
    }
    input.trim().parse().expect("Failed to parse input")
}

fn generate() -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
    let mut rng = rand::thread_rng();
    let mut row = 0;
    let mut col = 0;
    while row < BOARD_SIZE {
        while col < BOARD_SIZE {
            let num = rng.gen_range(0..10);
            if num == 0 {
                board[row][col] = num;
                col += 1;
                continue;
            }
            if is_num_valid(board, row, col, num) {
                board[row][col] = num;
            }
            col += 1;
        }
        row += 1;
        col = 0;
    }
    board
}

fn is_num_valid(board: [[i32; BOARD_SIZE]; BOARD_SIZE], row: usize, col: usize, num: i32) -> bool {
    let mut i = 0;
    while i < BOARD_SIZE {
        if board[row][i] == num {
            return false;
        }
        i += 1;
    }
    let mut j = 0;
    while j < BOARD_SIZE {
        if board[j][col] == num {
            return false;
        }
        j += 1;
    }
    let mut k = 0;
    let mut l = 0;
    while k < 3 {
        while l < 3 {
            if board[k + row - row % 3][l + col - col % 3] == num {
                return false;
            }
            l += 1;
        }
        k += 1;
        l = 0;
    }
    true
}

fn print_board(board: [[i32; BOARD_SIZE]; BOARD_SIZE]) {
    let mut i = 0;
    let mut j = 0;
    for row in board.iter() {
        for col in row.iter() {
            if *col == 0 {
                print!(" . ");
            } else {
                print!(" {} ", col);
            }
            i += 1;
            if i % 3 == 0 {
                print!(" ");
            }
        }
        if *row == [0; BOARD_SIZE] {
            print!(" . ");
        } else {
            println!();
        }
        j += 1;
        if j % 3 == 0 {
            println!();
        }
    }
}
