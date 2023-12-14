# Using official rust base image
FROM rust:1.74.1-slim-bookworm

# Set the application directory
WORKDIR /app

# # Install musl-tools to make many crates compile successfully
# RUN apk add --no-cache musl-dev
ENV RUST_LOG=TRACE

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the files to the Docker image
COPY ./ ./

