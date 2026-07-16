#!/bin/sh
set -eu

GIT_REVISION=$(git rev-parse --short HEAD)
APP_VERSION=$(awk -F'"' '/^\[package\]/{p=1} p && /^version *=/{print $2; exit}' Cargo.toml)
BUILD_DATE=$(date -u +%Y-%m-%dT%H:%M:%SZ)
CONTAINER_IMAGE=ghcr.io/hagatasdelus/glas

docker buildx build --push \
    --platform linux/amd64,linux/arm64 \
    --build-arg GIT_REVISION="${GIT_REVISION}" \
    --build-arg BUILD_DATE="${BUILD_DATE}" \
    --build-arg VERSION="${APP_VERSION}" \
    -t "${CONTAINER_IMAGE}:latest" -t "${CONTAINER_IMAGE}:${APP_VERSION}" \
    -f Containerfile \
    .
