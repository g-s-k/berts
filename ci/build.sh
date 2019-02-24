#!/bin/sh

cargo +$RUST web build --release -p beet-up-www
cargo +$RUST build --verbose
cargo +$RUST test --verbose
