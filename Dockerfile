﻿FROM rust:1-bookworm as builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates ^
 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-fastly-logs-analyzer /usr/local/bin/rust-fastly-logs-analyzer
ENTRYPOINT ["/usr/local/bin/rust-fastly-logs-analyzer"]
