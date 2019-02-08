FROM rust:latest
WORKDIR /usr/src/up/www

RUN cargo install cargo-web

CMD cargo web start --host=0.0.0.0
