# ---------- Build stage ----------
FROM rust:latest AS builder

WORKDIR /app

ENV SQLX_OFFLINE=true

# Copy everything
COPY . .

# Build release binary
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# Runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder stage
COPY --from=builder /app/target/release/doodoo-logistics-rust /app/app

EXPOSE 8080

CMD ["./app"]