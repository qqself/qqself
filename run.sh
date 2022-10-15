#!/bin/bash

set -euo pipefail

ACTION=""
if [[ -n "${1:-}" ]]; then ACTION=$1; fi
PARAM=""
if [[ -n "${2:-}" ]]; then PARAM=$2; fi
VERSION=${VERSION:-$(date +%s)}

log() { echo "[$(date)] $1"; }

usage() {
  echo "Usage:
    ./run.sh build  [FOLDER] 
    ./run.sh test   [FOLDER] 
    ./run.sh deploy [FOLDER]
  Commands by default will run action for all projects. Optionally you can pass [FOLDER] to filter out all the rest"
  exit 1
}

build() {
  log "Building all Rust projects"
  cargo build --frozen
}

test() {
  log "Testing all Rust projects"
  cargo test --frozen
}

deploy() {
    log "Deploying api-sync"    
    repo="public.ecr.aws/z9w5n5h3"
    region="us-east-1"
    tag="$repo/qqself-api-sync:$VERSION"
    docker build . --file api-sync/Dockerfile --tag $tag
    aws ecr-public get-login-password --region "$region" | docker login --username AWS --password-stdin "$repo"
    docker push $tag
}

if [[ "$ACTION" == "build" ]]; then
  build
elif [[ "$ACTION" == "test" ]]; then
  test
elif [[ "$ACTION" == "deploy" ]]; then
  deploy  
else
  usage
fi
