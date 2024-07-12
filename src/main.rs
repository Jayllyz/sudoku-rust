use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use std::sync::Mutex;
use tera::{Context, Tera};
pub mod sudoku;

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

#[allow(dead_code)]
async fn update_table(
    tera: web::Data<Tera>,
    app_state: web::Data<Sudoku>,
    difficulty: web::Path<usize>,
) -> impl Responder {
    let difficulty = difficulty.into_inner();
    let board = sudoku::generate_board(BOARD_SIZE, difficulty);
    app_state.set_board(board.clone());

    let mut context = Context::new();
    context.insert("title", "Sudoku-rust");
    context.insert("rows", &board);
    let template = tera
        .render("pages/index.html", &context)
        .expect("Error during rendering");

    HttpResponse::Ok().body(template)
}

#[allow(dead_code)]
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
        let app_state = web::Data::new(Sudoku {
            board: Mutex::new(vec![vec![0; BOARD_SIZE]; BOARD_SIZE]),
        });
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
    async fn test_solve_table() {
        let tera = web::Data::new(TEMPLATES.clone());
        let app_state = web::Data::new(Sudoku {
            board: Mutex::new(sudoku::generate_board(BOARD_SIZE, 1)),
        });
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
