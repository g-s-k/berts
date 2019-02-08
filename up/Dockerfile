FROM rust:latest
WORKDIR /usr/src/up

RUN cargo install cargo-watch

CMD cargo watch -x 'run ../db/tests/test.db'
