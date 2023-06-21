# Use a base image with the latest version of Rust installed
FROM rust:latest

# Set environment variables
ENV DATABASE_URL="postgres://local:password@postgres:5432/local" \
    PORT=8000 \
    ORIGIN="http://host.docker.internal:5000" \
    ACCESS_TOKEN_SECRET="" \
    REFRESH_TOKEN_SECRET="" \
    AWS_ACCESS_KEY_ID="" \
    AWS_SECRET_ACCESS_KEY="" \
    AWS_BUCKET_NAME="" \
    AWS_REGION="" \
    MINIO=true \
    ENDPOINT="http://minio:9000"

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