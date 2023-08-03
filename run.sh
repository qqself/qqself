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
    ./run.sh build [ client-web | api-sync | www ]
    ./run.sh test   
    ./run.sh deploy 
    ./run.sh lint   
    ./run.sh deps"
  exit 1
}

# Install dependencies and required tooling for the development
deps() {
  cargo fetch
  (cd client-web && yarn install) 
}

# Builds everything
build() {
  log "Building all Rust projects"
  cargo build --release --all-features
  log "Building client-web - core"
  (cd client-web && yarn build)
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
}

# Run linters and other static checkers
lint() {
  log "Linting all the Rust projects"
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  log "Linting all TypeScript projects"
  (cd client-web && yarn lint:check)
}

# Build a new Docker container and push to the registry
docker_push() {
  local repo="$1"
  local name="$2"
  local dockerfile="$3"
  tag="$repo/$name"
  region="us-east-1"
  docker build . --file "$dockerfile" --tag "$tag"
  aws ecr-public get-login-password --region "$region" | docker login --username AWS --password-stdin "$repo"
  docker push "$tag"
}

apprunner_update() {
  local service="$1"
  local tag="$2"
  # Initiate deployment: Get ARN of AWS AppRunner service and update it's config to point to the new Docker image tag
  arn=$(aws apprunner list-services --query "ServiceSummaryList[?ServiceName=='$service'].ServiceArn" --output text)   
  config="{\"ImageRepository\":{\"ImageIdentifier\":\"$tag\",\"ImageRepositoryType\":\"ECR_PUBLIC\"}}"
  aws apprunner update-service --service-arn "$arn" --source-configuration "$config" > /dev/null # Remove too noisy output
}

deploy() {
  local service="$1"
  log "Deploying $service"
  if [[ "$service" == "api-sync" ]]; then
    repo="public.ecr.aws/q1q1x2u3"
    name="qqself-api-sync:$VERSION"
    docker_push "$repo" "$name" "api-sync/Dockerfile" 
    apprunner_update "qqself-api-sync" "$repo/$name"
  elif [[ "$service" == "client-web" ]]; then 
    (cd client-web && yarn build)
    repo="public.ecr.aws/m5h4l2c6"
    name="qqself-app:$VERSION"
    docker_push "$repo" "$name" "client-web/Dockerfile"
    apprunner_update "qqself-client-web" "$repo/$name" 
  elif [[ "$service" == "www" ]]; then 
    repo="public.ecr.aws/q2c2s6b5"
    name="qqself-www:$VERSION"
    docker_push "$repo" "$name" "www/Dockerfile"
    apprunner_update "qqself-www" "$repo/$name" 
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
  "ci") build && lint && test ;;
  *) usage ;;
esac    
