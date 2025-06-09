FROM rust:1.82 AS builder
WORKDIR /usr/src/app
COPY Cargo.toml .
COPY services services
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/product-management /usr/local/bin/product-management
COPY --from=builder /usr/src/app/target/release/order-management /usr/local/bin/order-management
COPY --from=builder /usr/src/app/target/release/inventory-management /usr/local/bin/inventory-management
CMD ["sh", "-c", "/usr/local/bin/${SERVICE_NAME}"]