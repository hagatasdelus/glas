git_revision := `git rev-parse --short HEAD`
app_version := `awk -F'"' '/^\[package\]/{p=1} p && /^version *=/{print $2; exit}' Cargo.toml`
build_date := `date -u +%Y-%m-%dT%H:%M:%SZ`
container_runner := "docker"
container_image := "ghcr.io/hagatasdelus/glas"

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

container-local:
    {{container_runner}} build \
        --build-arg GIT_REVISION={{git_revision}} \
        --build-arg BUILD_DATE={{build_date}} \
        --build-arg VERSION={{app_version}} \
        -t {{container_image}}:latest -t {{container_image}}:{{app_version}} \
        -f Containerfile \
        .

container:
    sh .github/scripts/build_docker.sh
