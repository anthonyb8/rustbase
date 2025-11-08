# Use the official Rust image as a base
FROM rust:1.89-slim-bookworm

# Install dependencies for building OpenSSL
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the source code
COPY src ./src
RUN cargo build --release

CMD ["./target/release/backend"]
