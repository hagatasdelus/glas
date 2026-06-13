default:
    @just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

check:
    cargo check --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features

verify: fmt check test

run:
    cargo run

clean:
    cargo clean

build:
    cargo build --release

install: build
    @rm -f ~/.local/bin/git-glas
    @cp -f target/release/glas ~/.local/bin/git-glas
