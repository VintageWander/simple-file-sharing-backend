cargo prisma migrate dev --name production && \
cargo prisma migrate deploy && \
clear && \
cargo build --release && \
clear && \
cargo run --release