
format:
    cargo fmt

lint:
    cargo clippy

test:
    cargo test

run-cli +args:
    cargo run --bin lt {{args}}
