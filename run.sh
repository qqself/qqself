#!/bin/bash

set -euo pipefail

ACTION=""
if [[ -n "${1:-}" ]]; then ACTION=$1; fi
PARAM=""
if [[ -n "${2:-}" ]]; then PARAM=$2; fi
VERSION=${GITHUB_SHA:-$(date +%s)}

log() { echo "[$(date)] $1"; }

usage() {
  echo "Usage:
    ./run.sh build 
    ./run.sh test   
    ./run.sh deploy [ client-web | api-entries | www ]
    ./run.sh lint   
    ./run.sh deps"
  exit 1
}

# Install dependencies and required tooling for the development
deps() {
  cargo fetch
  (cd client-web && yarn install) 
  # cargo-lambda is needed for packaging and cross-compiling AWS Lambda for arm64
  pip3 install cargo-lambda
}

# Builds everything
build() {
  log "Building all Rust projects"
  cargo build --release --all-features
  log "Building client-web - core"
  (cd client-web && yarn build)
  if [[ "$(uname -s)" == "Darwin" ]]; then 
    (cd client-ios && make build)
  fi
}

# Run all the tests
test() {
  log "Testing all Rust projects"
  cargo test --release

  log "Testing WebAssembly"
  # Rely on wasm-pack coming with client-web
  (cd core && ../client-web/node_modules/.bin/wasm-pack test --release --node --features wasm)

  log "Testing all Typescript projects"
  (cd client-web && yarn test)

  # Skipping client-ios as for some reason Simulator is crashing on my local machine
}

# Run linters and other static checkers
lint() {
  log "Linting all the Rust projects"
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  log "Linting all TypeScript projects"
  (cd client-web && yarn lint:check)
  if [[ "$(uname -s)" == "Darwin" ]]; then 
    (cd client-ios && make lint-check)
  fi
}

s3_site_sync() {
  local files=$1
  local bucket=$2
  if [ ! -f "$files/index.html" ]; then
    echo "Error: No index.html file found in $files"
    exit 1
  fi
  aws s3 sync "$files" "$bucket" --delete
}

deploy_lambdas() {
  lambdas=("health" "delete" "find" "set")
  log "Buidling all lambdas"
  for lambda in "${lambdas[@]}"; do
    (cd "api-entries/lambda-$lambda" && cargo lambda build --arm64 --release --output-format zip)
  done
  log "Deploying all lambdas"
  for lambda in "${lambdas[@]}"; do
    aws lambda update-function-code --function-name "entries-$lambda" --zip-file fileb://./target/lambda/qqself-lambda-entries-$lambda/bootstrap.zip
  done
}

deploy() {
  local service="$1"
  log "Deploying $service"
  if [[ "$service" == "api-entries" ]]; then
    deploy_lambdas
  elif [[ "$service" == "client-web" ]]; then 
    (cd client-web && yarn build)
    s3_site_sync "client-web/dist" "s3://qqself-site-app" 
  elif [[ "$service" == "www" ]]; then 
    (cd www && docker run -u "$(id -u):$(id -g)" -v $PWD:/app --workdir /app ghcr.io/getzola/zola:v0.17.1 build)
    s3_site_sync "www/public" "s3://qqself-site-www"
  else 
    log "Specify what to deploy: api-sync | client-web | www"
  fi
}

case "$ACTION" in
  "build") build ;;
  "test") test ;;
  "deploy") deploy "$PARAM" ;;
  "deps") deps ;;
  "lint") lint ;;
  "ci") 
    build
    lint 
    test 
    ;;
  *) usage ;;
esac
