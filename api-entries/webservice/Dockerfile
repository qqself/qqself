# Should be build from the root of the project because workspace, vendor, etc.
FROM rust:1.75.0-slim-buster
WORKDIR /app
COPY . .
RUN cargo build --release --package qqself-api-sync --features dynamodb

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=0 /app/target/release/qqself-api-sync /app
CMD ["/app"]

