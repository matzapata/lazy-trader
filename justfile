
fmt:
    cargo fmt

lint:
    cargo clippy

test:
    cargo test

run-cli +args:
    cargo run --bin lt {{args}}

build:
    cargo build --release

install: build
    sudo mv target/release/lt /usr/local/bin

    