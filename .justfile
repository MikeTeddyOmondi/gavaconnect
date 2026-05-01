set shell := ["zsh", "-cu"]

# list available recipes
default:
    @just --list

# format code
fmt:
    cargo fmt --all

# check formatting without modifying files
fmt-check:
    cargo fmt --all --check

# run clippy (warnings = errors)
clippy:
    cargo clippy --all-targets --locked -- -D warnings

# run tests
test:
    cargo test --locked

# fmt → clippy → test (mirrors CI)
ci: fmt-check clippy test

# build dev
build:
    cargo build

# build release
build-release:
    cargo build --release
