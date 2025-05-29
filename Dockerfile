# --- Stage 1: Build the Rust binary ---
FROM rust:latest AS builder

# Install dependencies (if needed, e.g. OpenSSL)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy only Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create empty src dir so cargo fetch works even if src is empty initially
RUN mkdir src

# Fetch dependencies (this will cache if Cargo.toml/Cargo.lock unchanged)
RUN cargo fetch

# Now copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# --- Stage 2: Minimal runtime image ---
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS support
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy release binary from builder
COPY --from=builder /app/target/release/castify /app/castify

# Expose port your app listens on
EXPOSE 8000

# Optional: Set environment variables (can override at runtime)
ENV RUST_LOG=info

# Run the binary
CMD ["/app/castify"]
