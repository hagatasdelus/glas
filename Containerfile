# Stage 1. Build an app
FROM rust:1.96.0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2. Build for runtime
FROM dhi.io/debian-base:trixie

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION
LABEL org.opencontainers.image.title="glas" \
      org.opencontainers.image.description="Git-aware ls command written in Rust" \
      org.opencontainers.image.url="https://hagatasdelus.github.io/glas" \
      org.opencontainers.image.source="https://github.com/hagatasdelus/glas" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.revision=${GIT_REVISION} \
      org.opencontainers.image.created=${BUILD_DATE} \
      org.opencontainers.image.license="MIT License"

COPY --from=builder /app/target/release/glas /app/glas
WORKDIR /opt
ENTRYPOINT [ "/app/glas" ]
