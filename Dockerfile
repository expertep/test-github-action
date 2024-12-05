# Use the official Rust image as the builder
FROM rust:1.73 AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo files first (to cache dependencies)
COPY Cargo.toml Cargo.lock ./

# Create an empty "src" directory to allow dependency resolution
RUN mkdir src && echo "// Temporary file" > src/main.rs

# Pre-fetch dependencies
RUN cargo build --release && rm -rf src

# Copy the actual source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Create a smaller final image
FROM debian:bullseye-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/actix_web_service .

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./actix_web_service"]