FROM rust:1.54.0-alpine3.14
COPY Cargo.* ./
COPY src src
RUN cargo build && cargo test