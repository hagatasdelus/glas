#!/bin/sh
set -e

TO_VERSION=$1

if [ -z "$TO_VERSION" ]; then
  echo "Error: TO_VERSION argument is required." >&2
  exit 1
fi

sed "s/^version = \".*\"/version = \"$TO_VERSION\"/" Cargo.toml > a ; mv a Cargo.toml

sed "s/\${VERSION}/$TO_VERSION/g" .github/templates/README.md > a ; mv a README.md
sed "s/\${VERSION}/$TO_VERSION/g" .github/templates/README_ja.md > a ; mv a README_ja.md
