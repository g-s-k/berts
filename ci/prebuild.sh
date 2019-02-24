#!/bin/sh

rustup target install $RUST

which cargo-web || cargo install cargo-web
rm -rvf target/wasm32/unknown-unknown
