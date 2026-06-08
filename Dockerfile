# ---------- Build stage ----------
    FROM rust:1.78 as builder

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
    
    # install runtime dependencies (OpenSSL often needed for Actix/SQLx)
    RUN apt-get update && apt-get install -y \
        ca-certificates \
        libssl3 \
        && rm -rf /var/lib/apt/lists/*
    
    # copy binary from builder
    COPY --from=builder /app/target/release/doodoo-logistics-rust /app/app
    
    # expose port (Render overrides anyway)
    EXPOSE 8080
    
    CMD ["./app"]