# Dockerfile

FROM rust:1.60 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image
FROM debian:buster-slim

# Set the working directory
WORKDIR /root/

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/nocoin .

# Expose necessary ports (e.g., 30303 for P2P)
EXPOSE 30303 30304

# Run the application
CMD ["./nocoin"]
