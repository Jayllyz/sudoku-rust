FROM rust:1-slim

WORKDIR /app

COPY . .

RUN cargo build

EXPOSE 8000

CMD ["cargo", "run"]
