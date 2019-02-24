#!/bin/sh

cargo web build --release -p beet-up-www
cargo build --verbose
cargo test --verbose