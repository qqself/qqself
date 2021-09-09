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

  log "Building all WebAssembly"
  (cd client-pwa && wasm-pack build --target web)

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
  log "Formatting all TS projects"
    for file in */package.json; do
        dir=$(dirname $file)
        log "Processing $dir"
        (cd $dir && prettier --write --no-semi --arrow-parens=avoid --no-bracket-spacing --print-width=100 "**/*.ts" )
    done
}

deploy() {
  prefix="qqself-"
  local registry=$1
  local version=$2
  for path in */app.yaml; do
      app=$(dirname $path)
      tag=""
      for dockerfile in $path/Dockerfile*; do
          log "Deploying $app"
          docker=$(basename $dockerfile)
          image="$registry/$prefix$app"
          tag="$image:$version"
          log "Building $tag"
          (cd "$app" && docker build --tag "$tag" . --file $(basename $dockerfile))
          docker push $tag
          tag="$(docker inspect "$tag" --format="{{range .RepoDigests}}{{.}}|{{end}}" | tr "|" "\n" | grep $image)"
          config=$(cat "$app/app.yaml" | sed "s|\[IMAGE\]|$tag|")
      done
      echo "$config" | kubectl apply -f -
      rolloutType="deployment"
      cmdRollout="kubectl rollout status $rolloutType/$app"
      cmdWatch="while true; do kubectl get pods --selector app=$app --watch || true && sleep 3; done"
      local res=0; (echo "$cmdRollout"; echo "$cmdWatch") | parallel --line-buffer --halt now,done=1 || res=$? && true
      if [[ $res -ne 0 ]]; then
          log "Deployment for $app failed"
          kubectl get pods
          kubectl logs --selector "app=$app" --since 5m
          exit $res
      fi
  done
}

if [[ "$ACTION" == "build" ]]; then
  build
elif [[ "$ACTION" == "lint" ]]; then
  lint
elif [[ "$ACTION" == "deploy" ]]; then
  deploy "$PARAM" "$VERSION"
else
  usage
fi
