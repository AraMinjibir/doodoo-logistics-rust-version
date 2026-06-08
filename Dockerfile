# ---------- Pre-compute Layer Dependencies ----------
FROM rust:1.89-slim AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

# ---------- Build Engine Cache ----------
FROM rust:1.89-slim AS builder
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json

# Builds and caches  dependencies completely!
RUN cargo chef cook --release --recipe-path recipe.json

# ---------- Compile App Binary ----------
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin doodoo-logistics-rust

# ---------- Production Runtime Stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# Install critical runtime certificates and symbols
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the lean, pre-compiled release binary
COPY --from=builder /app/target/release/doodoo-logistics-rust /app/doodoo-logistics

EXPOSE 8080

CMD ["./doodoo-logistics"]