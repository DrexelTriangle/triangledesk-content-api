FROM rust:1.75-buster

WORKDIR /app

RUN cargo init .
COPY Cargo.* .

RUN cargo build --release

COPY ./src ./src/

RUN touch ./src/main.rs
RUN cargo build --release

CMD ["cargo", "run", "--release"]
