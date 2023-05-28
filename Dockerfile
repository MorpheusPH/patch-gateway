FROM rust:1.61.0 as builder
WORKDIR /usr/src/patch-gateway
COPY . .
RUN cargo install --path .
FROM debian:stable-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/patch-gateway /usr/local/bin/patch-gateway

ENTRYPOINT ["patch-gateway"]