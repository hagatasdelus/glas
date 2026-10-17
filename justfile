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

coverage:
    cargo llvm-cov --all-features --lcov --output-path coverage.lcov
    octocov

verify: fmt check test

run:
    cargo run

clean:
    cargo clean
    @rm -rf completions coverage.lcov

build:
    cargo build --release

install: build
    @rm -f ~/.local/bin/git-glas
    @cp -f target/release/glas ~/.local/bin/git-glas

credits:
    cargo about generate about.hbs -o CREDITS.md

prerelease: credits
    cargo check
    git add CHANGELOG.md CREDITS.md Cargo.toml Cargo.lock
