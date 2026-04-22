default:
    @just --list

fmt:
    cargo fmt --all

check:
    cargo check --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features

verify: fmt check test

clean:
    cargo clean

build:
    cargo build --release

install:
    @rm -f ~/.local/bin/git-glas
    @cp -f target/release/glas ~/.local/bin/git-glas
