use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use std::sync::Mutex;
use sudoku_rust::sudoku::Board;
use tera::{Context, Tera};

const BOARD_SIZE: usize = 9;

struct Sudoku {
    pub board: Mutex<Board>,
}

impl Sudoku {
    fn set_board(&self, board: Board) {
        *self.board.lock().unwrap() = board;
    }

    fn get_board(&self) -> Board {
        self.board.lock().unwrap().clone()
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        if let Err(e) = tera.load_from_glob("templates/**/*.html") {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[get("/")]
async fn home(tera: web::Data<Tera>) -> impl Responder {
    let board = Board::new(BOARD_SIZE);
    let mut context = Context::new();
    context.insert("title", "Sudoku-rust");
    context.insert("rows", board.rows());
    let template = tera.render("pages/index.html", &context).expect("Error");
    HttpResponse::Ok().body(template)
}

#[allow(dead_code)]
async fn update_table(
    tera: web::Data<Tera>,
    app_state: web::Data<Sudoku>,
    difficulty: web::Path<usize>,
) -> impl Responder {
    let difficulty = difficulty.into_inner();
    let board = Board::generate(BOARD_SIZE, difficulty);
    app_state.set_board(board.clone());

    let mut context = Context::new();
    context.insert("title", "Sudoku-rust");
    context.insert("rows", board.rows());
    let template = tera.render("pages/index.html", &context).expect("Error during rendering");

    HttpResponse::Ok().body(template)
}

#[allow(dead_code)]
async fn solve_table(tera: web::Data<Tera>, data: web::Data<Sudoku>) -> impl Responder {
    let mut board = data.get_board();
    board.resolv_backtrack();
    data.set_board(board.clone());

    let mut context = Context::new();
    context.insert("title", "Sudoku-rust");
    context.insert("rows", board.rows());
    let template = tera.render("pages/index.html", &context).expect("Error during rendering");

    HttpResponse::Ok().body(template)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Sudoku { board: Mutex::new(Board::new(BOARD_SIZE)) });
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
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_home() {
        let tera = web::Data::new(TEMPLATES.clone());
        let app = test::init_service(App::new().app_data(tera.clone()).service(home)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_update_table() {
        let tera = web::Data::new(TEMPLATES.clone());
        let app_state = web::Data::new(Sudoku { board: Mutex::new(Board::new(BOARD_SIZE)) });
        let app = test::init_service(
            App::new()
                .app_data(tera.clone())
                .app_data(app_state.clone())
                .service(web::resource("/update/{difficulty}").route(web::post().to(update_table))),
        )
        .await;

        let req = test::TestRequest::post().uri("/update/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_update_table_stores_generated_board() {
        let tera = web::Data::new(TEMPLATES.clone());
        let app_state = web::Data::new(Sudoku { board: Mutex::new(Board::new(BOARD_SIZE)) });

        let _ = update_table(tera, app_state.clone(), web::Path::from(1usize)).await;

        let board = app_state.get_board();
        assert_eq!(board.size(), BOARD_SIZE);
        assert!(
            board.rows().iter().flatten().any(|&x| x != 0),
            "update_table should replace the stored board with a generated one"
        );
    }

    #[actix_rt::test]
    async fn test_solve_table_stores_solved_board() {
        let tera = web::Data::new(TEMPLATES.clone());
        let app_state =
            web::Data::new(Sudoku { board: Mutex::new(Board::generate(BOARD_SIZE, 1)) });

        let _ = solve_table(tera, app_state.clone()).await;

        assert!(app_state.get_board().is_solved(), "solve_table should store the solved board");
    }

    #[actix_rt::test]
    async fn test_solve_table() {
        let tera = web::Data::new(TEMPLATES.clone());
        let app_state =
            web::Data::new(Sudoku { board: Mutex::new(Board::generate(BOARD_SIZE, 1)) });
        let app = test::init_service(
            App::new()
                .app_data(tera.clone())
                .app_data(app_state.clone())
                .service(web::resource("/solve").route(web::post().to(solve_table))),
        )
        .await;

        let req = test::TestRequest::post().uri("/solve").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
