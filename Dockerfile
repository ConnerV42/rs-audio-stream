FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner

COPY . .
# Create lockfile
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json /app/recipe.json 

# Build app dependencies
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Build app
ENV SQLX_OFFLINE true
RUN cargo build --release --bin audio_streamer

FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates to verify TLS certificates when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the binary from the builder stage
COPY --from=builder /app/target/release/audio_streamer /app
COPY configuration configuration
ENV ENV production
CMD ["./audio_streamer"]

