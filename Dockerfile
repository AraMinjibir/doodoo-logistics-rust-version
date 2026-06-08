# ---------- Build stage ----------
FROM rust:latest AS builder

WORKDIR /app

# 1. copy manifests first (for caching)
COPY Cargo.toml Cargo.lock ./

# 2. copy source
COPY src ./src

# 3. build release binary
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# copy binary from builder stage
COPY --from=builder /app/target/release/doodoo-logistics-rust /app/app

EXPOSE 8080

CMD ["./app"]