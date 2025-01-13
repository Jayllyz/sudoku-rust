FROM rust:1.84 AS base
RUN cargo install cargo-chef
WORKDIR /app

FROM base AS dev
RUN cargo install cargo-watch
COPY . .
CMD ["cargo", "watch", "-x", "run"]

FROM base AS planner
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
COPY --from=build /app/styles /app/styles
COPY --from=build /app/templates /app/templates
ENTRYPOINT ["/usr/local/bin/sudoku-rust"]
