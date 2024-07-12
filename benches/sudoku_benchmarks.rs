use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use sudoku_rust::sudoku::{generate_board, is_num_valid, resolv_backtrack};

const BOARD_SIZE: usize = 9;

lazy_static! {
    static ref EASY_BOARD: Vec<Vec<usize>> = vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];
    static ref MEDIUM_BOARD: Vec<Vec<usize>> = vec![
        vec![0, 0, 0, 2, 6, 0, 7, 0, 1],
        vec![6, 8, 0, 0, 7, 0, 0, 9, 0],
        vec![1, 9, 0, 0, 0, 4, 5, 0, 0],
        vec![8, 2, 0, 1, 0, 0, 0, 4, 0],
        vec![0, 0, 4, 6, 0, 2, 9, 0, 0],
        vec![0, 5, 0, 0, 0, 3, 0, 2, 8],
        vec![0, 0, 9, 3, 0, 0, 0, 7, 4],
        vec![0, 4, 0, 0, 5, 0, 0, 3, 6],
        vec![7, 0, 3, 0, 1, 8, 0, 0, 0]
    ];
    static ref HARD_BOARD: Vec<Vec<usize>> = vec![
        vec![0, 0, 0, 6, 0, 0, 4, 0, 0],
        vec![7, 0, 0, 0, 0, 3, 6, 0, 0],
        vec![0, 0, 0, 0, 9, 1, 0, 8, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 5, 0, 1, 8, 0, 0, 0, 3],
        vec![0, 0, 0, 3, 0, 6, 0, 4, 5],
        vec![0, 4, 0, 2, 0, 0, 0, 6, 0],
        vec![9, 0, 3, 0, 0, 0, 0, 0, 0],
        vec![0, 2, 0, 0, 0, 0, 1, 0, 0]
    ];
}

fn benchmark_generate_board(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_board");

    group.bench_function("9x9 easy", |b| {
        b.iter(|| generate_board(black_box(BOARD_SIZE), black_box(1)))
    });

    group.bench_function("9x9 medium", |b| {
        b.iter(|| generate_board(black_box(BOARD_SIZE), black_box(2)))
    });

    group.bench_function("9x9 hard", |b| {
        b.iter(|| generate_board(black_box(BOARD_SIZE), black_box(3)))
    });

    group.finish();
}

fn benchmark_resolv_backtrack(c: &mut Criterion) {
    let mut group = c.benchmark_group("resolv_backtrack");

    group.bench_function("easy", |b| {
        b.iter(|| {
            let mut board = EASY_BOARD.clone();
            resolv_backtrack(black_box(&mut board), 0, 0)
        })
    });

    group.bench_function("medium", |b| {
        b.iter(|| {
            let mut board = MEDIUM_BOARD.clone();
            resolv_backtrack(black_box(&mut board), 0, 0)
        })
    });

    group.bench_function("hard", |b| {
        b.iter(|| {
            let mut board = HARD_BOARD.clone();
            resolv_backtrack(black_box(&mut board), 0, 0)
        })
    });

    group.finish();
}

fn benchmark_is_num_valid(c: &mut Criterion) {
    let board = HARD_BOARD.clone();

    c.bench_function("is_num_valid", |b| {
        b.iter(|| {
            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    for num in 1..=BOARD_SIZE {
                        black_box(is_num_valid(
                            black_box(&board),
                            black_box(row),
                            black_box(col),
                            black_box(num),
                        ));
                    }
                }
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_generate_board,
    benchmark_resolv_backtrack,
    benchmark_is_num_valid
);
criterion_main!(benches);
