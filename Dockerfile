# Use a base image with the latest version of Rust installed
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Build the project and binaries
RUN cargo build --release -p prisma-cli; \
    target/release/prisma-cli generate; \
    cargo build --release;

FROM debian:stable-slim

# Transfer binaries to a new layer
COPY --from=builder /app/cert/                                       /cert/
COPY --from=builder /app/prisma/                                     /prisma/
COPY --from=builder /app/target/release/simple-file-sharing-backend  /usr/local/bin/simple-file-sharing-backend
COPY --from=builder /app/target/release/prisma-cli                   /usr/local/bin/prisma-cli

RUN apt update
RUN apt install -y libssl-dev ca-certificates

RUN prisma-cli