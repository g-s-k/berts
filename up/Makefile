all: build

CARGO_WATCH_INSTALLED := $(shell cargo-watch --version 2>/dev/null)
CARGO_WEB_INSTALLED := $(shell cargo-web --version 2>/dev/null)

install-watch:
ifndef CARGO_WATCH_INSTALLED
	cargo install cargo-watch
endif

install-web:
ifndef CARGO_WEB_INSTALLED
	cargo install cargo-web
endif

build-client: install-web
	cargo web deploy -p beet-up-www

build-server:
	cargo build

build: build-client build-server

start: install-watch install-web
	cargo watch -w .. -x 'web deploy -p beet-up-www' -x 'run -- ../db/tests/test.db'
