# Choose a base image with Rust installed
FROM rust:1.70.0 as builder

WORKDIR /server

RUN cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres
ENV SQLX_OFFLINE true

COPY configuration /server/configuration
COPY src /server/src
COPY Cargo.lock Cargo.toml sqlx-data.json /server/

RUN cargo build --release

# Use a lighter base image for the final layer
FROM debian:buster-slim

# Install openssl and postgresql-client for runtime
RUN apt-get update && apt-get install -y openssl postgresql-client && rm -rf /var/lib/apt/lists/*

WORKDIR /server

# Copy the binary from the builder stage
COPY --from=builder /server/configuration /server/configuration
COPY --from=builder /server/target/release/audio_streamer /server

# Install the necessary certificates to make HTTPS requests
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

ENV ENV production
CMD ["./audio_streamer"]

