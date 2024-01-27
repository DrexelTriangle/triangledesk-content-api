FROM rust:1.75-buster

WORKDIR /app

RUN cargo init .
COPY Cargo.* .

RUN cargo build --release
RUN rm -rf ./src

COPY ./src .

RUN ls -la

CMD ["cargo", "run", "--release"]
