# Using official rust base image
FROM rust:1.74.1-slim
ENV RUST_LOG=TRACE
# Set the application directory
WORKDIR /app
RUN apt update
# Install musl-tools to make many crates compile successfully
RUN apt install -y pkg-config openssl libssl-dev

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the files to the Docker image
COPY ./ ./

