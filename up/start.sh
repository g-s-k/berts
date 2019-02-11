#!/bin/sh

which cargo-watch || cargo install cargo-watch

cargo watch \
      -x 'web deploy --package beet-up-www' \
      -x 'run -- ../db/tests/test.db'
