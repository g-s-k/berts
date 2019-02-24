#!/bin/sh

WASM_TARGET=target/wasm32-unknown-unknown/release

rustup component add clippy
mkdir -p $WASM_TARGET
touch $WASM_TARGET/beet-up-www.{js,wasm}
