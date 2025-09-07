FROM rust:1.81-slim

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev libpq-dev build-essential curl lldb \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

WORKDIR /workspace
