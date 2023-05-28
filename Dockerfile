# Rust builder
FROM lukemathwalker/cargo-chef:latest-rust-1.69 AS chef
WORKDIR /usr/patch-gateway

FROM chef as planner
COPY Cargo.toml Cargo.toml
COPY rust-toolchain.toml rust-toolchain.toml
COPY src src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/patch-gateway/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.toml
COPY rust-toolchain.toml rust-toolchain.toml
COPY src src
RUN cargo build --release

FROM alpine:latest as base

COPY --from=builder /usr/patch-gateway/target/release/patch-gateway /usr/local/bin/patch-gateway

FROM base

ENTRYPOINT ["patch-gateway"]