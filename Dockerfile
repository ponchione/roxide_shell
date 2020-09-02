FROM rust:1.46-alpine

WORKDIR /usr/src/roxide

COPY . .

RUN cargo build --release

CMD cargo run