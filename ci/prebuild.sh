#!/bin/sh

rustup toolchain install $RUST

which cargo-web || cargo install cargo-web
