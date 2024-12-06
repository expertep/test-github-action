# Stage 1: Build
# FROM rust:alpine AS builder
FROM rust:1.73-slim AS builder
# Install necessary development tools
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the project files
COPY . .

RUN cargo check
RUN cargo test

RUN cargo fetch
# Build the project in release mode
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim


# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/test-github-action .

EXPOSE 8080
# Set the default command
CMD ["./test-github-action"]
# CMD ["ls"]