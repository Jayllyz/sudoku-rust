use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use rand::Rng;
use std::sync::Mutex;
use tera::{Context, Tera};

const BOARD_SIZE: usize = 9;
const SQUARE_SIZE: usize = 3;

struct Sudoku {
    pub board: Mutex<Vec<Vec<usize>>>,
}

impl Sudoku {
    fn set_board(&self, board: Vec<Vec<usize>>) {
        *self.board.lock().unwrap() = board;
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[get("/")]
async fn home(tera: web::Data<Tera>) -> impl Responder {
    let empty_board = vec![vec![0; BOARD_SIZE]; BOARD_SIZE];
    let mut context = Context::new();
    context.insert("title", "Sudoku-rust");
    context.insert("rows", &empty_board);
    let template = tera.render("pages/index.html", &context).expect("Error");
    HttpResponse::Ok().body(template)
}

async fn update_table(
    tera: web::Data<Tera>,
    app_state: web::Data<Sudoku>,
    difficulty: web::Path<usize>,
) -> impl Responder {
    let difficulty = difficulty.into_inner();
    let board = generate(BOARD_SIZE, difficulty);
    app_state.set_board(board.clone());

    let mut context = Context::new();
    context.insert("title", "Sudoku-rust");
    context.insert("rows", &board);
    let template = tera
        .render("pages/index.html", &context)
        .expect("Error during rendering");

    HttpResponse::Ok().body(template)
}

async fn solve_table(tera: web::Data<Tera>, data: web::Data<Sudoku>) -> impl Responder {
    let mut board = data.board.lock().unwrap().clone();
    resolv_backtrack(&mut board, 0, 0);
    data.set_board(board.clone());

    let mut context = Context::new();
    context.insert("title", "Sudoku-rust");
    context.insert("rows", &board);
    let template = tera
        .render("pages/index.html", &context)
        .expect("Error during rendering");

    HttpResponse::Ok().body(template)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Sudoku {
        board: Mutex::new(vec![vec![0; BOARD_SIZE]; BOARD_SIZE]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(web::Data::new(TEMPLATES.clone()))
            .service(
                actix_files::Files::new("/static", "./static/")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(home)
            .service(
                web::resource("/update/{difficulty}")
                    .route(web::post().to(update_table))
                    .app_data(app_state.clone()),
            )
            .service(
                web::resource("/solve")
                    .route(web::post().to(solve_table))
                    .app_data(app_state.clone()),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

pub fn generate(size: usize, difficulty: usize) -> Vec<Vec<usize>> {
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
