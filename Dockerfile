# Use a base image with the latest version of Rust installed
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Expose port 8000 to the outside world
EXPOSE 8000

# Install and build dependencies
RUN apt-get update; \
    apt-get upgrade -y; \
    apt-get install -y ca-certificates curl gnupg; \
    mkdir -p /etc/apt/keyrings; \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg; \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_21.x nodistro main" | \
    tee /etc/apt/sources.list.d/nodesource.list; \
    apt-get update; \
    apt-get install -y nodejs; \
    npm install -g local-ssl-proxy; \
    cargo prisma generate; \ 
    cargo build --release;