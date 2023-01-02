FROM rust:1-buster AS builder
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
COPY Cargo.toml.smol Cargo.toml
COPY Cargo.lock .
COPY src/bin/dummy.rs /app/src/bin/dummy.rs
RUN cargo build --release --bin dummy
COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release --bin ssr_server --features=ssr
COPY index.html .
RUN trunk build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=builder /app/target/release/ssr_server /ssr_server
COPY ./dist/ /dist/
ENTRYPOINT [ "/ssr_server", "--dir", "/dist/" ]