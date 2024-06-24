# Sudoku-rust 🧩

![ci](https://github.com/jayllyz/sudoku-rust/actions/workflows/ci.yml/badge.svg)

Personal project to learn Rust and improve my algorithm skills.
Recently started learning Rust and wanted to build something to practice because i really like the language.

## Roadmap 🗺️

- Generate a random sudoku board ✅
- Choose between 3 difficulties (easy, medium, hard) ✅
- Sudoku solver ✅
- local web app ✅
- Playable game in web app 🚧

## Run Locally 🚀

Clone the project

```bash
  git clone https://github.com/Jayllyz/sudoku-rust.git
```

Go to the project directory

```bash
  cd sudoku-rust
```

Build the docker image

```bash
  # with live reload
  docker build -t sudoku-rust --target=dev .

  # release optimized
  docker build -t sudoku-rust --target=prod .
```

Run the docker container

```bash
  # with live reload
  docker run --rm --name sudoku-rust -it -p 8000:8000 -v $(pwd):/app sudoku-rust

  # release optimized
  docker run --name sudoku-rust -it -p 8000:8000 sudoku-rust
```

## Author 👨‍💻

- [@Jayllyz](https://www.github.com/jayllyz)

## License ⚖️

[The Unlicense](https://choosealicense.com/licenses/unlicense/)
