use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use std::sync::Mutex;
use tera::{Context, Tera};
mod sudoku;

const BOARD_SIZE: usize = 9;
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
    let board = sudoku::generate(BOARD_SIZE, difficulty);
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
    sudoku::resolv_backtrack(&mut board, 0, 0);
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
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::sudoku::{generate, resolv_backtrack};

    #[test]
    fn board_valid() {
        const BOARD_SIZE: usize = 9;
        let board = generate(BOARD_SIZE, 1);
        assert_eq!(board.len(), 9);

        let mut hm = std::collections::HashMap::new();
        for row in board.iter().take(BOARD_SIZE).enumerate() {
            for value in row.1.iter().take(BOARD_SIZE) {
                if hm.contains_key(value) {
                    panic!("Invalid board");
                }
                if *value != 0 {
                    hm.insert(*value, true);
                }
            }
            hm.clear();
        }
        assert!(resolv_backtrack(&mut board.clone(), 0, 0));
    }
}
