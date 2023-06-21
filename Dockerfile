# Use a base image with the latest version of Rust installed
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

RUN cargo prisma generate
RUN cargo prisma migrate dev --name production
RUN cargo prisma migrate deploy

# Build the Rust application
RUN cargo build --release

# Specify the command to run when the container starts
CMD ["./target/release/simple-file-sharing-backend"]