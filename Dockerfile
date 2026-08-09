FROM rust:1.97.1-slim-bullseye AS builder
WORKDIR /usr/src/abit

COPY . .

RUN cargo build --package server --release

FROM debian:bookworm-slim

# Install necessary runtime dependencies for HTTPS requests
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/abit/target/release/server /app/abit-app
ENTRYPOINT ["/app/abit-app"]