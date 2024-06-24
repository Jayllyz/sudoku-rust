FROM rust:1 AS base
WORKDIR /app

FROM base AS dev 
RUN cargo install cargo-watch
COPY . .
CMD ["cargo", "watch", "-x", "run"]

FROM base AS planner
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM base AS build
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin sudoku-rust


FROM debian:stable-slim AS prod
WORKDIR /app
COPY --from=build /app/target/release/sudoku-rust /usr/local/bin
ENTRYPOINT ["/usr/local/bin/sudoku-rust"]
