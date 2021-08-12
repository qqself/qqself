#!/bin/bash

set -euo pipefail

ACTION=""
if [[ -n "${1:-}" ]]; then ACTION=$1; fi
PARAM=""
if [[ -n "${2:-}" ]]; then PARAM=$2; fi

log() { echo "[$(date)] $1"; }

usage() {
  echo "Usage:
    ./run.sh build # Build everything and run unit tests
    ./run.sh lint  # Format everything and run linter"
  exit 1
}

build() {
  log "Building all Rust projects"
  for file in */Cargo.toml; do
        dir=$(dirname $file)
        log "Processing $dir"
        (cd $dir && cargo build && cargo test)
  done

  log "Building all docker images"
  for file in */Dockerfile; do
      log "Processing $file"
      dir=$(dirname $file)
      (cd $dir && docker build .)
  done
}

lint() {
  log "Formatting all Rust projects"
  for file in */Cargo.toml; do
      dir=$(dirname $file)
      log "Processing $dir"
      (cd $dir && cargo fmt)
  done
}

if [[ "$ACTION" == "build" ]]; then
  build
elif [[ "$ACTION" == "lint" ]]; then
  lint
else
  usage
fi