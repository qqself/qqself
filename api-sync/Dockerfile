# Should be build from the root of the project because workspace, vendor, etc.
FROM rust:1.64.0-slim-buster
WORKDIR /app
COPY . .
RUN cargo build --frozen --release --package qqself-api-sync && \
    cargo test  --frozen --release --package qqself-api-sync

FROM debian:buster-slim
COPY --from=0 /app/target/release/qqself-api-sync /app
CMD ["/app"]

