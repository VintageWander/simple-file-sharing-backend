# Use a base image with the latest version of Rust installed
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

RUN curl -fsSL https://deb.nodesource.com/setup_21.x | bash - && \
    apt install -y nodejs; \
    npm install -g prisma; \
    npm install -g local-ssl-proxy;

# Build the project and binaries
RUN cargo build --release -p prisma-cli; \
    target/release/prisma-cli generate; \
    cargo build --release;

FROM debian:stable-slim

# Transfer binaries to a new layer
COPY --from=builder /usr/bin/                                        /usr/bin/
COPY --from=builder /usr/lib/node_modules/npm/                       /usr/lib/node_modules/npm/
COPY --from=builder /usr/lib/node_modules/prisma/                    /usr/lib/node_modules/prisma/
COPY --from=builder /usr/lib/node_modules/local-ssl-proxy/           /usr/lib/node_modules/local-ssl-proxy/
COPY --from=builder /app/cert/                                       /cert/
COPY --from=builder /app/prisma/                                     /prisma/
COPY --from=builder /app/target/release/prisma-cli                   /usr/local/bin/prisma-cli
COPY --from=builder /app/target/release/simple-file-sharing-backend  /usr/local/bin/simple-file-sharing-backend

RUN apt update
RUN apt install -y libssl-dev ca-certificates

RUN prisma-cli