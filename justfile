
format:
    cargo fmt

lint:
    cargo clippy

run-cli +args:
    cargo run --bin cli {{args}}
