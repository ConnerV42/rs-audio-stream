# Choose a base image with Rust installed
FROM rust:1.70.0 as builder

WORKDIR /server

COPY src /server/src
COPY tests /server/tests

COPY Cargo.lock Cargo.toml /server/

# Install sqlx-cli for setting up the database
RUN cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres

# Build your Rust application in release mode
RUN cargo build --release

# Use a lighter base image for the final layer
FROM debian:buster-slim

# Install openssl and postgresql-client for runtime
RUN apt-get update && apt-get install -y openssl postgresql-client && rm -rf /var/lib/apt/lists/*

WORKDIR /server

# Copy the binary from the builder stage
COPY --from=builder /server/target/release/audio_streamer /server

# Install the necessary certificates to make HTTPS requests
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

CMD ["./audio_streamer"]

